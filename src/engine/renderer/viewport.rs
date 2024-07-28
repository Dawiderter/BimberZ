use super::context::GraphicsContext;

pub struct ViewportSurface {
    pub(super) surface: wgpu::Surface<'static>,
    pub(super) config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
}

impl ViewportSurface {
    pub fn resize(&mut self, ctx: &GraphicsContext, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&ctx.device, &self.config);
        }
    }
}
