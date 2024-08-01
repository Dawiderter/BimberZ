use std::{collections::HashMap, sync::Arc};

use tracing::{error, info};
use winit::event::{Event, WindowEvent};

use super::{
    egui_integration::BimberzEguiState,
    fps_counter::FPSCounter,
    input::Input,
    renderer::{
        context::GraphicsContext, scene::Scene, uniforms::Uniforms, viewport::ViewportSurface,
        Renderer,
    },
};

pub struct Viewport {
    pub window: Arc<winit::window::Window>,
    pub surface: ViewportSurface,
}

pub struct Window {
    viewports: HashMap<winit::window::WindowId, Viewport>,
    main_window_id: winit::window::WindowId,
    event_loop: winit::event_loop::EventLoop<()>,
    fps_counter: FPSCounter,
    renderer: Renderer,
    input: Input,
    egui_state: BimberzEguiState,
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

        let renderer = Renderer::new(ctx);
        let input = Input::new();

        let egui_state = BimberzEguiState::from_root(window.clone());

        let main_window_id = window.id();
        let main_viewport = Viewport {
            window,
            surface: main_viewport_surface,
        };

        let viewports = HashMap::from([(main_window_id, main_viewport)]);

        Self {
            event_loop,
            renderer,
            input,
            egui_state,
            main_window_id,
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
                    let response = self.egui_state.apply_event(window_id, event);
                    if response.consumed {
                        tracing::info!("Egui consumed this event: {:?}", event);
                        return;
                    }
                }

                // TODO: Make input register only main window events
                self.input.register_event(&event);

                match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => {
                        // TODO: Close only one window instead of the whole app
                        info!("The close button was pressed; stopping");
                        elwt.exit();
                    }
                    Event::WindowEvent {
                        event: WindowEvent::Resized(new_size),
                        window_id,
                    } => {
                        // TODO: Apply new surface configurations before drawing, not on every resize
                        self.viewports
                            .get_mut(&window_id)
                            .unwrap()
                            .surface
                            .resize(&self.renderer.ctx, new_size);
                    }
                    Event::AboutToWait => {
                        // IMPROVEMENT: Maybe don't redraw windows that don't need it
                        for viewport in self.viewports.values() {
                            viewport.window.request_redraw();
                        }
                    }
                    Event::WindowEvent {
                        event: WindowEvent::RedrawRequested,
                        window_id,
                    } => {
                        if window_id == self.main_window_id {
                            let viewport = &self.viewports[&window_id];

                            let egui_input = self.egui_state.take_input(&window_id);

                            let egui_ctx = &self.egui_state.ctx;

                            self.fps_counter.advance_frame();

                            let egui_output = egui_ctx.run(egui_input, |ctx| {
                                f(
                                    &mut self.input,
                                    &mut self.renderer.uniforms,
                                    &mut self.renderer.scene,
                                    ctx,
                                );

                                ctx.show_viewport_deferred(
                                    egui::ViewportId::from_hash_of("Diagnostics"),
                                    egui::ViewportBuilder::default()
                                        .with_title("Diagnostics")
                                        .with_inner_size((400.0, 300.0)),
                                    move |_, _| {},
                                );
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

                            self.egui_state
                                .handle_platform_output(&window_id, egui_output.platform_output);

                            let egui_viewport_delta = self
                                .egui_state
                                .handle_viewport_output(egui_output.viewport_output, elwt);

                            // TODO: Delay window creation and deletion to avoid crashing when trying to handle already queued events related to this window
                            for window in egui_viewport_delta.added {
                                let surface = self.renderer.ctx.create_surface(window.clone());
                                self.viewports
                                    .insert(window.id(), Viewport { window, surface });
                            }
                            for window in egui_viewport_delta.removed {
                                self.viewports.remove(&window.id());
                                assert!(Arc::strong_count(&window) == 1);
                            }
                        } else {
                            // HACK: Fix by doing above TODO
                            let Some(viewport) = self.viewports.get(&window_id) else {
                                info!("Window not found");
                                return;
                            };

                            let mut egui_input = self.egui_state.take_input(&window_id);
                            if let Some(new_window) =
                                self.egui_state
                                    .apply_prev_output(&window_id, &mut egui_input, elwt)
                            {
                                self.viewports.remove(&window_id);
                                let surface = self.renderer.ctx.create_surface(new_window.clone());
                                self.viewports.insert(
                                    new_window.id(),
                                    Viewport {
                                        window: new_window,
                                        surface,
                                    },
                                );
                                return;
                            }
                            let egui_ctx = &self.egui_state.ctx;

                            let egui_output = if egui_input.viewport_id
                                == egui::ViewportId::from_hash_of("Diagnostics")
                            {
                                egui_ctx.run(egui_input, |ctx| {
                                    egui::Window::new("FPS").show(ctx, |ui| {
                                        ui.label(format!("FPS: {:.1}", self.fps_counter.fps()));
                                        if self.fps_counter.curr_duration().as_secs() >= 5 {
                                            self.fps_counter.reset();
                                        }
                                    });
                                })
                            } else {
                                egui_ctx.run(egui_input, |_| {
                                    self.egui_state.call(&window_id);
                                })
                            };

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

                            self.egui_state
                                .handle_platform_output(&window_id, egui_output.platform_output);
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
