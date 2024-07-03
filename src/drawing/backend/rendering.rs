use std::{borrow::Cow, num::NonZeroU64};

use tracing::info;

use crate::drawing::renderer::bindings::{self, Bindings};

pub struct RenderState {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub pipeline: wgpu::RenderPipeline,
    pub vertex_shader: wgpu::ShaderModule,
    pub shader: wgpu::ShaderModule,
    pub bindings_bind_group: wgpu::BindGroup,
    pub bindings_bind_group_layout: wgpu::BindGroupLayout,
    pub bindings_buffer: wgpu::Buffer,
    pub size: winit::dpi::PhysicalSize<u32>,
}

impl RenderState {
    pub async fn init(window: &winit::window::Window, frame_width: u32, frame_height: u32) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::default();
        let surface = unsafe { instance.create_surface(window).unwrap() };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        info!("{:?}", adapter.get_info());

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("BimberZ Device"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
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
        };

        surface.configure(&device, &config);

        let bindings_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry { 
                        binding: 0, 
                        visibility: wgpu::ShaderStages::FRAGMENT, 
                        ty: wgpu::BindingType::Buffer { 
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false, 
                            min_binding_size: None 
                        }, 
                        count: None 
                    }
                ],
                label: Some("Bindings Bind Group Layout"),
            });

        let bindings_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Bindings Buffer"),
            size: 1,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bindings_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bindings_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding { 
                        buffer: &bindings_buffer, 
                        offset: 0, 
                        size: None,
                    }),
                }
            ],
            label: Some("Bindings Bind Group"),
        });

        let vertex_shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&bindings_bind_group_layout],
                push_constant_ranges: &[],
            });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            pipeline,
            vertex_shader,
            shader,
            bindings_bind_group,
            bindings_bind_group_layout,
            bindings_buffer
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn update_bindings(&mut self, bindings: &mut Bindings) {
        if bindings.need_rebinding {
            bindings.need_rebinding = false;

            info!("Rebinding bindings");

            let bindings_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Bindings Buffer"),
                size: bindings.used_slots() as u64 * bindings::BIND_BYTES,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

            let bindings_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &self.bindings_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding { 
                            buffer: &bindings_buffer, 
                            offset: 0, 
                            size: NonZeroU64::new(bindings::BIND_BYTES * bindings.used_slots() as u64),
                        }),
                    }
                ],
                label: Some("Bindings Bind Group"),
            });

            self.bindings_buffer = bindings_buffer;
            self.bindings_bind_group = bindings_bind_group;
        }

        let data_locked = bindings.bindings.read().unwrap();
        let data = bytemuck::cast_slice(&data_locked);

        self.queue.write_buffer(&self.bindings_buffer, 0, data)
    }

    pub fn set_shader(&mut self, shader_src: &str) {
        let shader = self.device.create_shader_module(
            wgpu::ShaderModuleDescriptor { 
                label: Some("Dynamically Loaded Shader"), 
                source: wgpu::ShaderSource::Wgsl(Cow::from(shader_src)) 
            }
        );
        self.shader = shader;
        self.update_pipeline();
    }

    fn update_pipeline(&mut self) {
        let render_pipeline_layout =
            self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&self.bindings_bind_group_layout],
                push_constant_ranges: &[],
            });

        let pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &self.vertex_shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &self.shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: self.config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        self.pipeline = pipeline;
    }

    pub fn render_routine(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("BimberZ Main Routine Encoder"),
            });

        encoder.push_debug_group("Render Routine");

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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

            render_pass.set_pipeline(&self.pipeline);
            render_pass.insert_debug_marker("Binding");
            render_pass.set_bind_group(0, &self.bindings_bind_group, &[]);
            render_pass.draw(0..4, 0..1);
        }

        encoder.pop_debug_group();

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
