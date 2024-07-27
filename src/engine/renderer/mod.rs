pub mod context;
pub mod egui_integration;
pub mod raymarcher;
pub mod scene;
pub mod uniforms;

use context::GraphicsContext;
use egui_integration::BimberzEgui;
use raymarcher::Raymarcher;
use scene::Scene;
use uniforms::Uniforms;
use winit::dpi::PhysicalSize;

pub struct Renderer {
    pub uniforms: Uniforms,
    pub scene: Scene,
    ctx: GraphicsContext,
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

    pub fn resize_surface(&mut self, new_size: PhysicalSize<u32>) {
        self.ctx.resize(new_size);
        self.egui.screen_descriptor.size_in_pixels = [new_size.width, new_size.height];
    }

    pub fn prepare_egui(
        &mut self,
        shapes: Vec<egui::ClippedPrimitive>,
        textures_delta: egui::TexturesDelta,
        pixels_per_point: f32,
    ) {
        self.egui.paint_jobs = shapes;
        self.egui.textures_delta = textures_delta;
        self.egui.screen_descriptor.pixels_per_point = pixels_per_point;
    }

    pub fn render_routine(&mut self) -> Result<(), wgpu::SurfaceError> {
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
}
