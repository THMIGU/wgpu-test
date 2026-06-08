use bytemuck::{Pod, Zeroable};
use glam::Mat4;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Uniform {
	uniform: [[f32; 4]; 4],
}

impl Uniform {
	pub fn new(uniform: Mat4) -> Self {
		Self {
			uniform: uniform.to_cols_array_2d(),
		}
	}
}
