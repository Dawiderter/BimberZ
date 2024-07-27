use std::sync::Arc;

use tracing::{error, info};
use winit::event::{Event, WindowEvent};

use super::{
    fps_counter::FPSCounter,
    input::Input,
    renderer::{context::GraphicsContext, scene::Scene, uniforms::Uniforms, Renderer},
};

pub struct Window {
    inner: Arc<winit::window::Window>,
    event_loop: winit::event_loop::EventLoop<()>,
    egui_state: egui_winit::State,
    fps_counter: FPSCounter,
    renderer: Renderer,
    input: Input,
}

impl Window {
    pub async fn new(frame_width: u32, frame_height: u32, scale: u32) -> Self {
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
            // will be written to stdout.
            .with_max_level(tracing::Level::INFO)
            // completes the builder.
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");

        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        let window = Arc::new(
            winit::window::WindowBuilder::new()
                .with_title("BimberZ")
                .with_inner_size(winit::dpi::PhysicalSize::new(
                    scale * frame_width,
                    scale * frame_height,
                ))
                .with_resizable(false)
                .build(&event_loop)
                .unwrap(),
        );

        let ctx = GraphicsContext::new(window.clone()).await;
        let renderer = Renderer::new(ctx);
        let input = Input::new();

        let egui_state = egui_winit::State::new(
            egui::Context::default(),
            egui::ViewportId::ROOT,
            &window,
            None,
            None,
        );

        Self {
            inner: window,
            event_loop,
            renderer,
            input,
            egui_state,
            fps_counter: FPSCounter::new(),
        }
    }

    pub fn run(mut self, mut f: impl FnMut(&mut Input, &mut Uniforms, &mut Scene)) {
        self.event_loop
            .set_control_flow(winit::event_loop::ControlFlow::Poll);

        self.event_loop
            .run(move |event, elwt| {
                self.input.register_event(&event);

                if let Event::WindowEvent { event, .. } = &event {
                    let response = self.egui_state.on_window_event(&self.inner, event);
                    if response.consumed {
                        tracing::info!("Egui consumed this event: {:?}", event);
                        return;
                    }
                }

                match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => {
                        info!("The close button was pressed; stopping");
                        elwt.exit();
                    }
                    Event::WindowEvent {
                        event: WindowEvent::Resized(new_size),
                        ..
                    } => {
                        self.renderer.ctx.resize(new_size);
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
                        f(
                            &mut self.input,
                            &mut self.renderer.uniforms,
                            &mut self.renderer.scene,
                        );

                        let mut egui_input = self.egui_state.take_egui_input(&self.inner);
                        let egui_ctx = self.egui_state.egui_ctx();

                        for viewport in egui_input.viewports.values_mut() {
                            egui_winit::update_viewport_info(viewport, egui_ctx, &self.inner, true);
                        }

                        let egui_output = egui_ctx.run(egui_input, |ctx| {
                            egui::Window::new("hello").show(ctx, |ui| {
                                ui.label("Hello World");
                                if ui.button("Click").clicked() {
                                    tracing::info!("Clicked!");
                                }
                            });
                        });

                        let clipped_primitives =
                            egui_ctx.tessellate(egui_output.shapes, egui_output.pixels_per_point);

                        self.renderer.prepare_egui(
                            &clipped_primitives,
                            &egui_output.textures_delta,
                            egui_output.pixels_per_point,
                        );

                        match self
                            .renderer
                            .render_routine(&clipped_primitives, egui_output.pixels_per_point)
                        {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => {}
                            Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                            Err(e) => {
                                error!("{:?}", e)
                            }
                        }

                        self.input.clear_tapped();

                        self.egui_state
                            .handle_platform_output(&self.inner, egui_output.platform_output);

                        self.fps_counter.advance_frame();
                        if self.fps_counter.curr_duration() >= std::time::Duration::from_secs(5) {
                            info!("FPS: {}", self.fps_counter.fps());
                            self.fps_counter.reset();
                        }
                    }
                    _ => (),
                }
            })
            .unwrap();
    }

    pub fn uniforms(&mut self) -> &mut Uniforms {
        &mut self.renderer.uniforms
    }

    pub fn scene(&mut self) -> &mut Scene {
        &mut self.renderer.scene
    }
}
