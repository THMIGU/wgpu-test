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

struct MaterialProperties {
	lit: u32
}

@group(0) @binding(1)
var<uniform> light_uniform: LightUniform;

@group(1) @binding(0)
var diffuse_texture: texture_2d<f32>;
@group(1) @binding(1)
var diffuse_sampler: sampler;
@group(1) @binding(2)
var<storage> material_properties: MaterialProperties;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	let base_color = textureSample(
		diffuse_texture,
		diffuse_sampler,
		in.uv
	);

	if material_properties.lit == 1 {
		let normal = in.normal;

		let direction = -light_uniform.direction;
		let light_color = light_uniform.color;
		let intensity = light_uniform.intensity;

		let n = normalize(normal);
		let l = normalize(direction);

		let diffuse = max(dot(n, l), 0.0);

		let ambient = 0.2;
		let lighting = min(intensity * diffuse + ambient, 1.0);

		let final_color = base_color.rgb * light_color * lighting;

		return vec4<f32>(final_color, base_color.a);
	} else {
		return base_color;
	}
}
