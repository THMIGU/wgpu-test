struct VertexInput {
	@location(0) position: vec3<f32>,
	@location(1) color: vec3<f32>,
};

struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(0) color: vec3<f32>,
};

struct Uniform {
	mat: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> view_uniform: Uniform;
@group(1) @binding(0)
var<uniform> model_uniform: Uniform;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
	var out: VertexOutput;

	out.position = view_uniform.mat * model_uniform.mat * vec4<f32>(
		input.position,
		1.0
	);
	out.color = input.color;

	return out;
}

@fragment
fn fs_main(@location(0) color: vec3<f32>) -> @location(0) vec4<f32> {
	return vec4<f32>(color, 1.0);
}
