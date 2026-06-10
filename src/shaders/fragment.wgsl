struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(0) normal: vec3<f32>,
	@location(1) uv: vec2<f32>,
};

struct LightUniform {
	direction: vec3<f32>,
	color: vec3<f32>,
	intensity: f32,
}

@group(0) @binding(1)
var<uniform> light_uniform: LightUniform;

@group(1) @binding(0)
var diffuse_texture: texture_2d<f32>;
@group(1) @binding(1)
var diffuse_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	let ambient = 0.2;
	let base_color = textureSample(
		diffuse_texture,
		diffuse_sampler,
		in.uv
	);

	let normal = in.normal;

	let direction = -light_uniform.direction;
	let light_color = light_uniform.color;
	let intensity = light_uniform.intensity;

	let n = normalize(normal);
	let l = normalize(direction);

	let diffuse = max(dot(n, l), 0.0);

	let final_color = base_color.rgb * light_color * (intensity * diffuse + ambient);
	return vec4<f32>(final_color, 1.0);
}
