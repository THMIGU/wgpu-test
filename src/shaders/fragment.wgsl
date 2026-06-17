struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(0) normal: vec3<f32>,
	@location(1) uv: vec2<f32>,
	@location(2) world_pos: vec3<f32>,
};

struct ViewUniform {
	mat: mat4x4<f32>,
	pos: vec3<f32>,
}

struct LightUniform {
	direction: vec3<f32>,
	color: vec3<f32>,
	intensity: f32,
}

struct MaterialProperties {
	lit: u32
}

@group(0) @binding(0)
var<uniform> view_uniform: ViewUniform;
@group(0) @binding(1)
var<uniform> light_uniform: LightUniform;

@group(1) @binding(0)
var diffuse_texture: texture_2d<f32>;
@group(1) @binding(1)
var diffuse_sampler: sampler;
@group(1) @binding(2)
var<uniform> material_properties: MaterialProperties;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	let base_color = textureSample(
		diffuse_texture,
		diffuse_sampler,
		in.uv
	);

	if material_properties.lit == 1 {
		let direction = -light_uniform.direction;
		let light_color = light_uniform.color;
		let intensity = light_uniform.intensity;

		let n = normalize(in.normal);
		let l = normalize(direction);
		let v = normalize(view_uniform.pos - in.world_pos);

		let h = normalize(l + v);
		let spec_strength = 8.0;
		let spec = pow(max(dot(n, h), 0.0), spec_strength);

		let diffuse = max(dot(n, l), 0.0);

		let ambient = 0.2;

		var lighting = ambient + diffuse * intensity + spec * intensity;
		// lighting = min(lighting, 1.0);

		let final_color = base_color.rgb * light_color * lighting;

		return vec4<f32>(final_color, base_color.a);
	} else {
		return base_color;
	}
}
