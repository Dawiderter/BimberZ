pub mod context;
pub mod raymarcher;
pub mod scene;
pub mod uniforms;

use context::GraphicsContext;
use raymarcher::Raymarcher;
use scene::Scene;
use uniforms::Uniforms;

pub struct Renderer {
    pub ctx: GraphicsContext,
    pub raymarcher: Raymarcher,
    pub uniforms: Uniforms,
    pub scene: Scene,
    pub egui: egui_wgpu::Renderer,
}

impl Renderer {
    pub fn new(mut ctx: GraphicsContext) -> Self {
        let raymarcher = Raymarcher::new(&mut ctx);
        let uniforms = Uniforms::new();
        let scene = Scene::new();
        let egui = egui_wgpu::Renderer::new(&ctx.device, ctx.surface_format, None, 1, false);

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
        shapes: &[egui::ClippedPrimitive],
        textures_delta: &egui::TexturesDelta,
        pixels_per_point: f32,
    ) {
        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("BimberZ Egui Drawing"),
            });

        self.egui.update_buffers(
            &self.ctx.device,
            &self.ctx.queue,
            &mut encoder,
            shapes,
            &egui_wgpu::ScreenDescriptor {
                size_in_pixels: [600, 600],
                pixels_per_point,
            },
        );

        for (texture_id, delta) in &textures_delta.set {
            self.egui
                .update_texture(&self.ctx.device, &self.ctx.queue, *texture_id, delta)
        }

        for texture_id in &textures_delta.free {
            self.egui.free_texture(texture_id);
        }

        self.ctx.queue.submit(std::iter::once(encoder.finish()));
    }

    pub fn render_routine(
        &mut self,
        shapes: &[egui::ClippedPrimitive],
        pixels_per_point: f32,
    ) -> Result<(), wgpu::SurfaceError> {
        self.prepare();

        let output = self.ctx.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("BimberZ Main Routine Encoder"),
            });

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

            self.render(&mut rpass);

            self.egui.render(
                &mut rpass,
                shapes,
                &egui_wgpu::ScreenDescriptor {
                    size_in_pixels: [600, 600],
                    pixels_per_point,
                },
            );
        }

        encoder.pop_debug_group();

        self.ctx.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    fn prepare(&mut self) {
        self.raymarcher
            .prepare(&mut self.ctx, &mut self.uniforms, &mut self.scene);
    }

    fn render(&mut self, rpass: &mut wgpu::RenderPass) {
        self.raymarcher.render(rpass);
    }
}
