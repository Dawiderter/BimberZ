
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    
    let x = f32(in_vertex_index & 1u) * 2.0 - 1.0;
    let y = f32(in_vertex_index & 2u) - 1.0;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);

    let x_tex = f32(in_vertex_index & 1u);
    let y_tex = 1.0 - f32((in_vertex_index & 2u) / 2u);
    out.tex_coords = vec2<f32>(x_tex,y_tex);

    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}

 