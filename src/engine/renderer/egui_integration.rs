use super::context::GraphicsContext;

pub struct BimberzEgui {
    pub(super) renderer: egui_wgpu::Renderer,
    pub(super) paint_jobs: Vec<egui::ClippedPrimitive>,
    pub(super) textures_delta: egui::TexturesDelta,
    pub(super) screen_descriptor: egui_wgpu::ScreenDescriptor,
}

impl BimberzEgui {
    pub(super) fn new(ctx: &mut GraphicsContext) -> Self {
        let renderer = egui_wgpu::Renderer::new(&ctx.device, ctx.surface_format, None, 1, false);

        let paint_jobs = Vec::new();
        let textures_delta = egui::TexturesDelta::default();
        let size = [0, 0];
        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: size,
            pixels_per_point: 1.0,
        };

        Self {
            renderer,
            paint_jobs,
            textures_delta,
            screen_descriptor,
        }
    }

    pub(super) fn prepare(
        &mut self,
        ctx: &mut GraphicsContext,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        self.renderer.update_buffers(
            &ctx.device,
            &ctx.queue,
            encoder,
            &self.paint_jobs,
            &self.screen_descriptor,
        );

        for (texture_id, delta) in &self.textures_delta.set {
            self.renderer
                .update_texture(&ctx.device, &ctx.queue, *texture_id, delta)
        }
    }

    pub(super) fn render<'rp, 's: 'rp>(&'s self, rpass: &mut wgpu::RenderPass<'rp>) {
        self.renderer
            .render(rpass, &self.paint_jobs, &self.screen_descriptor);
    }

    pub(super) fn free(&mut self) {
        for texture_id in &self.textures_delta.free {
            self.renderer.free_texture(texture_id);
        }

        self.paint_jobs.clear();
        self.textures_delta.clear();
    }
}
