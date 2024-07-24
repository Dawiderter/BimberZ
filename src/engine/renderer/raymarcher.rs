use std::borrow::Cow;

use super::{
    context::GraphicsContext,
    scene::Scene,
    uniforms::{UniformValue, Uniforms, UNIFORM_SIZE},
};

pub struct Raymarcher {
    pipeline_layout: wgpu::PipelineLayout,
    pipeline: wgpu::RenderPipeline,
    vertex_shader: wgpu::ShaderModule,
    fragment_shader: wgpu::ShaderModule,
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
    uniform_buffer: wgpu::Buffer,
    prep_buffer: Vec<u8>,
    user_data_desc: String,
    scene_desc: String,
}

impl Raymarcher {
    pub fn new(ctx: &mut GraphicsContext) -> Self {
        let bind_group_layout =
            ctx.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: Some("Raymarcher Bind Group Layout"),
                });
        let pipeline_layout = ctx
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let uniform_buffer = Self::create_buffer(ctx);
        let bind_group = Self::create_bind_group(ctx, &bind_group_layout, &uniform_buffer);
        let vertex_shader = ctx
            .device
            .create_shader_module(wgpu::include_wgsl!("shaders/base.wgsl"));

        let fragment_shader = ctx
            .device
            .create_shader_module(wgpu::include_wgsl!("shaders/base.wgsl"));

        let pipeline =
            Self::create_pipeline(ctx, &pipeline_layout, &vertex_shader, &fragment_shader);

        let prep_buffer = Vec::new();

        let user_data_desc = String::from("_p: u8");
        let scene_desc = String::from("0.0");

        Self {
            pipeline_layout,
            pipeline,
            vertex_shader,
            fragment_shader,
            bind_group,
            bind_group_layout,
            uniform_buffer,
            prep_buffer,
            user_data_desc,
            scene_desc,
        }
    }

    pub fn prepare(
        &mut self,
        ctx: &mut GraphicsContext,
        uniforms: &mut Uniforms,
        scene: &mut Scene,
    ) {
        let buffer_size = uniforms.bytes_len();

        self.prep_buffer.clear();
        self.prep_buffer.resize(buffer_size, 0);

        uniforms.write_to_buffer(&mut self.prep_buffer);

        if uniforms.has_changed_structure {
            tracing::info!("Rebinding uniforms");

            let new_buffer = Self::create_init_buffer(ctx, &self.prep_buffer);
            let new_bind_group = Self::create_bind_group(ctx, &self.bind_group_layout, &new_buffer);

            self.uniform_buffer = new_buffer;
            self.bind_group = new_bind_group;

            self.user_data_desc = Self::uniforms_to_wgsl_struct_fields(uniforms);
        } else {
            ctx.queue
                .write_buffer(&self.uniform_buffer, 0, &self.prep_buffer);
        }

        if scene.has_changed {
            self.scene_desc = scene.to_wgsl();
        }

        if uniforms.has_changed_structure || scene.has_changed {
            self.rebuild_shader(ctx);
        }

        uniforms.has_changed_structure = false;
        scene.has_changed = false;
    }

    pub fn render(&self, rpass: &mut wgpu::RenderPass) {
        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, &self.bind_group, &[]);
        rpass.draw(0..4, 0..1);
    }

    fn rebuild_shader(&mut self, ctx: &mut GraphicsContext) {
        tracing::info!("Rebuilding shader");

        let shader_code = Self::build_shader_code(&self.user_data_desc, &self.scene_desc);

        let new_shader = ctx
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Raymarching Shader"),
                source: wgpu::ShaderSource::Wgsl(Cow::Owned(shader_code)),
            });

        let new_pipeline =
            Self::create_pipeline(ctx, &self.pipeline_layout, &self.vertex_shader, &new_shader);

        self.fragment_shader = new_shader;
        self.pipeline = new_pipeline;
    }

    fn create_buffer(ctx: &mut GraphicsContext) -> wgpu::Buffer {
        ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Raymarcher Uniform Buffer"),
            size: 1,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn create_init_buffer(ctx: &mut GraphicsContext, data: &[u8]) -> wgpu::Buffer {
        use wgpu::util::DeviceExt;
        ctx.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Raymarcher Uniform Buffer"),
                contents: data,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            })
    }

    fn create_bind_group(
        ctx: &mut GraphicsContext,
        layout: &wgpu::BindGroupLayout,
        buffer: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer,
                    offset: 0,
                    size: None,
                }),
            }],
            label: Some("Raymarcher Bind Group"),
        })
    }

    fn create_pipeline(
        ctx: &mut GraphicsContext,
        layout: &wgpu::PipelineLayout,
        vertex_shader: &wgpu::ShaderModule,
        fragment_shader: &wgpu::ShaderModule,
    ) -> wgpu::RenderPipeline {
        ctx.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(layout),
                vertex: wgpu::VertexState {
                    module: vertex_shader,
                    entry_point: "vs_main",
                    buffers: &[],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: fragment_shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: ctx.config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
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
                cache: None,
            })
    }

    fn uniforms_to_wgsl_struct_fields(uniforms: &Uniforms) -> String {
        use std::fmt::Write;

        let mut desc = String::new();

        for (key, value) in uniforms.iter_with_keys() {
            let size = value.size();
            let i = key.idx();
            let kind = match value {
                UniformValue::F32(_) => "f32",
                UniformValue::Vec2(_) => "vec2f",
                UniformValue::Vec3(_) => "vec3f",
            };
            writeln!(desc, "s{i} : {kind},").unwrap();
            let padding_left = (UNIFORM_SIZE - size) / 4;
            for _ in 0..padding_left {
                writeln!(desc, "_p : u32,").unwrap();
            }
        }

        desc
    }

    fn build_shader_code(user_data_struct: &str, scene: &str) -> String {
        const TEMPLATE_SHADER: &str = include_str!("shaders/template.wgsl");

        TEMPLATE_SHADER
            .replace("{{USER_DATA}}", user_data_struct)
            .replace("{{SCENE}}", scene)
    }
}

#[cfg(test)]
mod tests {
    use glam::{vec2, vec3};

    use super::*;

    #[test]
    fn uniform_to_wgsl_struct_test() {
        let mut uniforms = Uniforms::new();
        let to_remove = uniforms.bind(4.0);
        uniforms.bind(vec2(1.0, 5.0));
        uniforms.bind(vec3(0.1, 0.2, 0.5));
        uniforms.bind(0.1);

        let wgsl_struct = Raymarcher::uniforms_to_wgsl_struct_fields(&uniforms);
        println!("{}", wgsl_struct);

        uniforms.unbind(to_remove);

        let wgsl_struct = Raymarcher::uniforms_to_wgsl_struct_fields(&uniforms);
        println!("{}", wgsl_struct);
    }
}
