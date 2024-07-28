use std::{collections::HashMap, sync::Arc};

use tracing::{error, info};
use winit::event::{Event, WindowEvent};

use super::{
    fps_counter::FPSCounter,
    input::Input,
    renderer::{
        context::GraphicsContext, scene::Scene, uniforms::Uniforms, viewport::ViewportSurface,
        Renderer,
    },
};

pub struct Viewport {
    window: Arc<winit::window::Window>,
    surface: ViewportSurface,
}

pub struct Window {
    viewports: HashMap<winit::window::WindowId, Viewport>,
    main_window: winit::window::WindowId,
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

        let (ctx, main_viewport_surface) = GraphicsContext::new(window.clone()).await;
        let secondary_window = Arc::new(
            winit::window::WindowBuilder::new()
                .with_title("Hello World")
                .with_inner_size(winit::dpi::PhysicalSize::new(400, 400))
                .with_resizable(false)
                .build(&event_loop)
                .unwrap(),
        );
        let secondary_viewport = Viewport {
            window: secondary_window.clone(),
            surface: ctx.create_surface(secondary_window.clone()),
        };

        let renderer = Renderer::new(ctx);
        let input = Input::new();

        let egui_ctx = egui::Context::default();
        egui_ctx.set_zoom_factor(0.7);

        let egui_state =
            egui_winit::State::new(egui_ctx, egui::ViewportId::ROOT, &window, None, None);

        let main_window = window.id();
        let main_viewport = Viewport {
            window,
            surface: main_viewport_surface,
        };

        let viewports = HashMap::from([
            (main_window, main_viewport),
            (secondary_window.id(), secondary_viewport),
        ]);

        Self {
            event_loop,
            renderer,
            input,
            egui_state,
            main_window,
            viewports,
            fps_counter: FPSCounter::new(),
        }
    }

    pub fn run(mut self, mut f: impl FnMut(&mut Input, &mut Uniforms, &mut Scene, &egui::Context)) {
        self.event_loop
            .set_control_flow(winit::event_loop::ControlFlow::Poll);

        self.event_loop
            .run(move |event, elwt| {
                if let Event::WindowEvent { event, window_id } = &event {
                    let response = self
                        .egui_state
                        .on_window_event(&self.viewports[window_id].window, event);
                    if response.consumed {
                        tracing::info!("Egui consumed this event: {:?}", event);
                        return;
                    }
                }

                self.input.register_event(&event);

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
                        window_id,
                    } => {
                        self.viewports
                            .get_mut(&window_id)
                            .unwrap()
                            .surface
                            .resize(&self.renderer.ctx, new_size);
                    }
                    Event::AboutToWait => {
                        // Application update code.

                        // Queue a RedrawRequested event.
                        //
                        // You only need to call this if you've determined that you need to redraw, in
                        // applications which do not always need to. Applications that redraw continuously
                        // can just render here instead.
                        for viewport in self.viewports.values() {
                            viewport.window.request_redraw();
                        }
                    }
                    Event::WindowEvent {
                        event: WindowEvent::RedrawRequested,
                        window_id,
                    } => {
                        if window_id == self.main_window {
                            let viewport = &self.viewports[&window_id];

                            let mut egui_input = self.egui_state.take_egui_input(&viewport.window);
                            let egui_ctx = self.egui_state.egui_ctx();
                            self.fps_counter.advance_frame();

                            for egui_viewport in egui_input.viewports.values_mut() {
                                egui_winit::update_viewport_info(
                                    egui_viewport,
                                    egui_ctx,
                                    &viewport.window,
                                    true,
                                );
                            }

                            let egui_output = egui_ctx.run(egui_input, |ctx| {
                                f(
                                    &mut self.input,
                                    &mut self.renderer.uniforms,
                                    &mut self.renderer.scene,
                                    ctx,
                                );

                                egui::Window::new("FPS").show(ctx, |ui| {
                                    ui.label(format!("FPS: {:.1}", self.fps_counter.fps()));
                                    if self.fps_counter.curr_duration().as_secs() >= 5 {
                                        self.fps_counter.reset();
                                    }
                                });
                            });

                            let clipped_primitives = egui_ctx
                                .tessellate(egui_output.shapes, egui_output.pixels_per_point);

                            self.renderer.prepare_egui(
                                clipped_primitives,
                                egui_output.textures_delta,
                                viewport.surface.size,
                                egui_output.pixels_per_point,
                            );

                            match self.renderer.render_routine(&viewport.surface) {
                                Ok(_) => {}
                                Err(wgpu::SurfaceError::Lost) => {}
                                Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                                Err(e) => {
                                    error!("{:?}", e)
                                }
                            }

                            self.input.clear_tapped();

                            self.egui_state.handle_platform_output(
                                &viewport.window,
                                egui_output.platform_output,
                            );
                        } else {
                            let viewport = &self.viewports[&window_id];

                            let mut egui_input = self.egui_state.take_egui_input(&viewport.window);
                            let egui_ctx = self.egui_state.egui_ctx();

                            for egui_viewport in egui_input.viewports.values_mut() {
                                egui_winit::update_viewport_info(
                                    egui_viewport,
                                    egui_ctx,
                                    &viewport.window,
                                    true,
                                );
                            }

                            let egui_output = egui_ctx.run(egui_input, |ctx| {
                                egui::Window::new("Hello").show(ctx, |ui| {
                                    ui.label("Hello world!");
                                });
                            });

                            let clipped_primitives = egui_ctx
                                .tessellate(egui_output.shapes, egui_output.pixels_per_point);

                            self.renderer.prepare_egui(
                                clipped_primitives,
                                egui_output.textures_delta,
                                viewport.surface.size,
                                egui_output.pixels_per_point,
                            );

                            match self.renderer.render_only_egui(&viewport.surface) {
                                Ok(_) => {}
                                Err(wgpu::SurfaceError::Lost) => {}
                                Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                                Err(e) => {
                                    error!("{:?}", e)
                                }
                            }

                            self.egui_state.handle_platform_output(
                                &viewport.window,
                                egui_output.platform_output,
                            );
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
