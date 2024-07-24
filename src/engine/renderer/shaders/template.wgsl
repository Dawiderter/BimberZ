
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

// This should work but naga's OpenGL backend ignores alignment
// So we need to manually insert padding
// struct UserData {
//     @align(16) color: f32,
//     @align(16) color_mul: f32
// }

struct UserData {
    {{USER_DATA}}
}

@group(0) @binding(0)
var<uniform> data: UserData;

fn sphere(p : vec3f, r : f32) -> f32 {
    return length(p) - r;
}

fn torus(p: vec3f, t: vec2f) -> f32 {
    let q = vec2(length(p.xy) - t.x, p.z);
    return length(q) - t.y;
}

fn box(p: vec3f, b: vec3f) -> f32 {
    let q = abs(p) - b;
    return length(max(q,vec3f(0.0))) + min(max(q.x,max(q.y,q.z)),0.0);
}

fn scene(p : vec3f) -> f32 {

    let d = {{SCENE}};

    return d;
}

const MAX_STEPS : i32 = 64;
const MIN_HIT_DIST : f32 = 0.001;
const MAX_TRACE_DIST : f32 = 1000.0;

fn ray_march(ro : vec3f, rd: vec3f) -> vec3f {
    var distance_traveled : f32 = 0.0;

    for (var i = 0; i < MAX_STEPS; i++) {
        let curr_pos = ro + distance_traveled * rd;

        let distance_to_closest = scene(curr_pos);

        if (distance_to_closest < MIN_HIT_DIST) {
            let normal = calc_normals(curr_pos);

            let light_pos = vec3(2.0, -5.0, 3.0);

            let light_dir = normalize(curr_pos - light_pos);

            let diff_intensity = 0.2 + max(0.0, dot(normal, light_dir)) * 0.8;

            return vec3(0.5, 0.5, 0.2) * diff_intensity;
        }

        if (distance_traveled > MAX_TRACE_DIST) {
            break;
        }

        distance_traveled += distance_to_closest;
    }

    return vec3(0.1, 0.0, 0.1);
}

fn calc_normals(p: vec3f) -> vec3f {
    let small_step = vec2(0.001, 0.0);

    let gradient_x = scene(p + small_step.xyy) - scene(p - small_step.xyy);
    let gradient_y = scene(p + small_step.yxy) - scene(p - small_step.yxy);
    let gradient_z = scene(p + small_step.yyx) - scene(p - small_step.yyx);

    let normal = vec3(gradient_x,  gradient_y, gradient_z);

    return normalize(normal);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = (vec2(in.uv.x, 1.0 - in.uv.y)) * 2.0 - 1.0;

    let camera_pos = vec3(0.0, 0.0, -5.0);
    let ro = camera_pos;
    let rd = vec3(uv, 1.0);

    let color = ray_march(ro, rd);

    return vec4(color, 0.0);
}
