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
}

impl Renderer {
    pub fn new(mut ctx: GraphicsContext) -> Self {
        let raymarcher = Raymarcher::new(&mut ctx);
        let uniforms = Uniforms::new();
        let scene = Scene::new();
        Self {
            ctx,
            raymarcher,
            uniforms,
            scene,
        }
    }

    pub fn render_routine(&mut self) -> Result<(), wgpu::SurfaceError> {
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
