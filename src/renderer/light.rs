use bytemuck::{Pod, Zeroable};
use glam::Vec3;

pub enum LightType {
	DirectionalLight(LightUniform),
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct LightUniform {
	direction: [f32; 3],
	_pad0: f32,
	color: [f32; 3],
	intensity: f32,
}

impl LightUniform {
	pub fn new(direction: Vec3, color: Vec3, intensity: f32) -> Self {
		Self {
			direction: direction.to_array(),
			_pad0: 0_f32,
			color: color.to_array(),
			intensity,
		}
	}
}
