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
    egui_state: HashMap<winit::window::WindowId, egui_winit::State>,
    egui_viewports: HashMap<egui::ViewportId, winit::window::WindowId>,
    main_window_id: winit::window::WindowId,
    event_loop: winit::event_loop::EventLoop<()>,
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

        let renderer = Renderer::new(ctx);
        let input = Input::new();

        let egui_ctx = egui::Context::default();
        egui_ctx.set_embed_viewports(false);
        egui_ctx.set_zoom_factor(0.7);

        let main_egui_state = egui_winit::State::new(
            egui_ctx.clone(),
            egui::ViewportId::ROOT,
            &window,
            None,
            None,
        );

        let main_window_id = window.id();
        let main_viewport = Viewport {
            window,
            surface: main_viewport_surface,
        };

        let viewports = HashMap::from([(main_window_id, main_viewport)]);
        let egui_state = HashMap::from([(main_window_id, main_egui_state)]);
        let egui_viewports = HashMap::from([(egui::ViewportId::ROOT, main_window_id)]);

        Self {
            event_loop,
            renderer,
            input,
            egui_state,
            main_window_id,
            viewports,
            egui_viewports,
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
                        .get_mut(window_id)
                        .unwrap()
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
                        if window_id == self.main_window_id {
                            let viewport = &self.viewports[&window_id];

                            let mut egui_input = self
                                .egui_state
                                .get_mut(&window_id)
                                .unwrap()
                                .take_egui_input(&viewport.window);

                            let egui_ctx = self.egui_state[&window_id].egui_ctx().clone();

                            self.fps_counter.advance_frame();

                            egui_winit::update_viewport_info(
                                egui_input
                                    .viewports
                                    .get_mut(&egui_input.viewport_id)
                                    .unwrap(),
                                &egui_ctx,
                                &viewport.window,
                                true,
                            );

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

                                ctx.show_viewport_deferred(
                                    egui::ViewportId::from_hash_of("Secondary"),
                                    egui::ViewportBuilder::default()
                                        .with_title("Diagnostics")
                                        .with_inner_size((400.0, 300.0)),
                                    move |_, _| {},
                                );

                                ctx.show_viewport_deferred(
                                    egui::ViewportId::from_hash_of("Tertiary"),
                                    egui::ViewportBuilder::default()
                                        .with_title("Diagnostics 2")
                                        .with_inner_size((500.0, 300.0)),
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
                                .get_mut(&window_id)
                                .unwrap()
                                .handle_platform_output(
                                    &viewport.window,
                                    egui_output.platform_output,
                                );

                            for (id, output) in egui_output.viewport_output {
                                if let std::collections::hash_map::Entry::Vacant(entry) =
                                    self.egui_viewports.entry(id)
                                {
                                    let window = Arc::new(
                                        egui_winit::create_window(&egui_ctx, elwt, &output.builder)
                                            .unwrap(),
                                    );
                                    let window_id = window.id();
                                    let state = egui_winit::State::new(
                                        egui_ctx.clone(),
                                        id,
                                        &window,
                                        None,
                                        None,
                                    );
                                    let surface = self.renderer.ctx.create_surface(window.clone());
                                    self.egui_state.insert(window_id, state);
                                    self.viewports
                                        .insert(window_id, Viewport { window, surface });
                                    entry.insert(window_id);
                                }
                            }
                        } else {
                            let viewport = &self.viewports[&window_id];

                            let mut egui_input = self
                                .egui_state
                                .get_mut(&window_id)
                                .unwrap()
                                .take_egui_input(&viewport.window);
                            let egui_ctx = self.egui_state[&window_id].egui_ctx();

                            egui_winit::update_viewport_info(
                                egui_input
                                    .viewports
                                    .get_mut(&egui_input.viewport_id)
                                    .unwrap(),
                                egui_ctx,
                                &viewport.window,
                                true,
                            );

                            let egui_output = if egui_input.viewport_id
                                == egui::ViewportId::from_hash_of("Secondary")
                            {
                                egui_ctx.run(egui_input, |ctx| {
                                    egui::Window::new("Hello").show(ctx, |ui| {
                                        ui.label("Hello world!");
                                    });
                                    egui::Window::new("What is happening???").show(ctx, |ui| {
                                        if ui.button("Why?").clicked() {
                                            tracing::info!("Clicked!");
                                        }
                                    });
                                })
                            } else {
                                egui_ctx.run(egui_input, |ctx| {
                                    egui::Window::new("Okay").show(ctx, |ui| {
                                        ui.label("Hello world!");
                                    });
                                    egui::Window::new("Interesting").show(ctx, |ui| {
                                        if ui.button("Why?").clicked() {
                                            tracing::info!("Clicked!");
                                        }
                                    });
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
                                .get_mut(&window_id)
                                .unwrap()
                                .handle_platform_output(
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
