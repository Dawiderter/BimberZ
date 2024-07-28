use std::sync::Arc;

use tracing::info;

use super::viewport::ViewportSurface;

pub struct GraphicsContext {
    pub(super) instance: wgpu::Instance,
    pub(super) device: wgpu::Device,
    pub(super) queue: wgpu::Queue,
    pub(super) surface_format: wgpu::TextureFormat,
}

impl GraphicsContext {
    pub async fn new(window: Arc<winit::window::Window>) -> (Self, ViewportSurface) {
        let size = window.inner_size();
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        info!("Chosen adapter {:?}", adapter.get_info());

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("BimberZ Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = *surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .unwrap_or(&surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let ctx = Self {
            device,
            instance,
            queue,
            surface_format,
        };

        let viewport = ViewportSurface {
            surface,
            config,
            size,
        };

        (ctx, viewport)
    }

    pub fn create_surface(&self, window: Arc<winit::window::Window>) -> ViewportSurface {
        let size = window.inner_size();

        let surface = self.instance.create_surface(window).unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&self.device, &config);

        ViewportSurface {
            surface,
            config,
            size,
        }
    }
}
