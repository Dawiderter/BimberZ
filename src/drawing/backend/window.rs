use winit::event::{Event, WindowEvent};

use crate::drawing::framebuffer::{self, FrameBuffer};

use super::rendering::RenderState;

pub struct Window {
    inner: winit::window::Window,
    event_loop: winit::event_loop::EventLoop<()>,
    render_state: RenderState,
    frame_buffer: FrameBuffer,
}

impl Window {
    pub async fn init(frame_width: u32, frame_height: u32, scale: u32) -> Self {
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        let window = winit::window::WindowBuilder::new()
            .with_title("BimberZ")
            .with_inner_size(winit::dpi::PhysicalSize::new(
                scale*frame_width, scale*frame_height,
            ))
            .with_resizable(false)
            .build(&event_loop)
            .unwrap();

        let render_state = RenderState::init(&window, frame_width, frame_height).await;

        let frame_buffer = FrameBuffer::new(frame_width, frame_height);

        Self {
            inner: window,
            event_loop,
            render_state,
            frame_buffer
        }
    }

    pub fn run(mut self, mut f : impl FnMut(&mut FrameBuffer)) {
        self.event_loop
            .set_control_flow(winit::event_loop::ControlFlow::Poll);

        self.event_loop
            .run(move |event, elwt| {
                match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => {
                        println!("The close button was pressed; stopping");
                        elwt.exit();
                    }
                    Event::WindowEvent {
                        event: WindowEvent::Resized(new_size),
                        ..
                    } => {
                        self.render_state.resize(new_size);
                    }
                    Event::AboutToWait => {
                        // Application update code.

                        // Queue a RedrawRequested event.
                        //
                        // You only need to call this if you've determined that you need to redraw, in
                        // applications which do not always need to. Applications that redraw continuously
                        // can just render here instead.
                        self.inner.request_redraw();
                    }
                    Event::WindowEvent {
                        event: WindowEvent::RedrawRequested,
                        ..
                    } => {

                        f(&mut self.frame_buffer);
                        self.render_state.write_buffer_to_screen(&self.frame_buffer);

                        match self.render_state.render_routine() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => {}
                            Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                            Err(e) => {} //error!("{:?}", e) }
                        }
                    }
                    _ => (),
                }
            })
            .unwrap();
    }
}
