//Vertex
struct CameraUniform {
    view_proj : mat4x4<f32>,
}

@group(0) @binding(0) var<uniform> camera : CameraUniform;

struct InstanceInput {
    @location(5)  model_matrix_0 : vec4<f32>,
    @location(6)  model_matrix_1 : vec4<f32>,
    @location(7)  model_matrix_2 : vec4<f32>,
    @location(8)  model_matrix_3 : vec4<f32>,
    @location(9)  color          : vec4<f32>,
    @location(10) finisher       : u32,
}

struct VertexInput {
    @location(0) pos : vec3<f32>,
    @location(1) uv  : vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_pos : vec4<f32>,
    @location(0)       uv       : vec2<f32>,
    @location(1)       color    : vec4<f32>,
    @location(2)       finisher : u32,
}

@vertex
fn vertex_main(in: VertexInput, instance: InstanceInput) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    var out: VertexOutput;
    out.clip_pos = camera.view_proj * model_matrix * vec4<f32>(in.pos, 1.0);
    out.uv       = in.uv;
    out.color    = instance.color;
    out.finisher = instance.finisher;

    return out;
}

// Fragment
@group(1) @binding(0) var t0 : texture_2d<f32>;
@group(1) @binding(1) var s0 : sampler;

@group(2) @binding(0) var t1 : texture_2d<f32>;
@group(2) @binding(1) var s1 : sampler;

@group(3) @binding(0) var t2 : texture_2d<f32>;
@group(3) @binding(1) var s2 : sampler;

@group(4) @binding(0) var t3 : texture_2d<f32>;
@group(4) @binding(1) var s3 : sampler;

@fragment
fn fragment_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let texture_finisher = textureSample(t2, s2, in.uv);
    let overlay_finisher = textureSample(t3, s3, in.uv);
    let texture = textureSample(t0, s0, in.uv);
    let overlay = textureSample(t1, s1, in.uv);
    if in.finisher == u32(1) {
        let out = overlay_finisher * overlay_finisher.a + (texture_finisher * in.color) * (1.0 - overlay_finisher.a);
        return out;
    } else {

        let out = overlay * overlay.a + (texture * in.color) * (1.0 - overlay.a);
        return out;
    }
}