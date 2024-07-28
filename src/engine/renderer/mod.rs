pub mod context;
pub mod egui_integration;
pub mod raymarcher;
pub mod scene;
pub mod uniforms;
pub mod viewport;

use context::GraphicsContext;
use egui_integration::BimberzEgui;
use raymarcher::Raymarcher;
use scene::Scene;
use uniforms::Uniforms;
use viewport::ViewportSurface;

pub struct Renderer {
    pub uniforms: Uniforms,
    pub scene: Scene,
    pub ctx: GraphicsContext,
    raymarcher: Raymarcher,
    egui: BimberzEgui,
}

impl Renderer {
    pub fn new(mut ctx: GraphicsContext) -> Self {
        let raymarcher = Raymarcher::new(&mut ctx);
        let uniforms = Uniforms::new();
        let scene = Scene::new();
        let egui = BimberzEgui::new(&mut ctx);

        Self {
            ctx,
            raymarcher,
            uniforms,
            scene,
            egui,
        }
    }

    pub fn prepare_egui(
        &mut self,
        shapes: Vec<egui::ClippedPrimitive>,
        textures_delta: egui::TexturesDelta,
        size: winit::dpi::PhysicalSize<u32>,
        pixels_per_point: f32,
    ) {
        self.egui.paint_jobs = shapes;
        self.egui.textures_delta = textures_delta;
        self.egui.screen_descriptor.size_in_pixels = [size.width, size.height];
        self.egui.screen_descriptor.pixels_per_point = pixels_per_point;
    }

    pub fn render_routine(&mut self, viewport: &ViewportSurface) -> Result<(), wgpu::SurfaceError> {
        let output = viewport.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("BimberZ Main Routine Encoder"),
            });

        self.raymarcher
            .prepare(&mut self.ctx, &mut self.uniforms, &mut self.scene);
        self.egui.prepare(&mut self.ctx, &mut encoder);

        encoder.push_debug_group("Render Routine");

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::RED),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.raymarcher.render(&mut rpass);
            self.egui.render(&mut rpass);
        }

        encoder.pop_debug_group();

        self.ctx.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        self.egui.free();

        Ok(())
    }

    pub fn render_only_egui(
        &mut self,
        viewport: &ViewportSurface,
    ) -> Result<(), wgpu::SurfaceError> {
        let output = viewport.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Egui Rendering Encoder"),
            });

        self.egui.prepare(&mut self.ctx, &mut encoder);

        encoder.push_debug_group("Egui Rendering");

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Egui Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::RED),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.egui.render(&mut rpass);
        }

        encoder.pop_debug_group();

        self.ctx.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        self.egui.free();

        Ok(())
    }
}
