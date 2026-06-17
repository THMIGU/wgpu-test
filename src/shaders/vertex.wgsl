struct VertexInput {
	@location(0) position: vec3<f32>,
	@location(1) normal: vec3<f32>,
	@location(2) uv: vec2<f32>,
};

struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(0) normal: vec3<f32>,
	@location(1) uv: vec2<f32>,
	@location(2) world_pos: vec3<f32>,
};

struct Uniform {
	mat: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> view_uniform: Uniform;

@group(2) @binding(0)
var<uniform> model_uniform: Uniform;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
	var out: VertexOutput;

	let world_pos = model_uniform.mat * vec4<f32>(
		input.position,
		1.0
	);

	out.position = view_uniform.mat * world_pos;
	out.normal = (model_uniform.mat * vec4<f32>(input.normal, 0.0)).xyz;
	out.uv = input.uv;
	out.world_pos = world_pos.xyz;

	return out;
}
