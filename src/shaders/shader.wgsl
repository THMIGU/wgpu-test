struct VertexInput {
	@location(0) position: vec3<f32>,
	@location(1) uv: vec2<f32>,
};

struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(0) uv: vec2<f32>,
};

struct Uniform {
	mat: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> view_uniform: Uniform;

@group(1) @binding(0)
var diffuse_texture: texture_2d<f32>;
@group(1) @binding(1)
var diffuse_sampler: sampler;

@group(2) @binding(0)
var<uniform> model_uniform: Uniform;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
	var out: VertexOutput;

	out.position = view_uniform.mat * model_uniform.mat * vec4<f32>(
		input.position,
		1.0
	);
	out.uv = input.uv;

	return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	return textureSample(
		diffuse_texture,
		diffuse_sampler,
		in.uv
	);
}
