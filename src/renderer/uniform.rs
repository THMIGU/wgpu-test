use bytemuck::{Pod, Zeroable};
use glam::Mat4;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Uniform {
	mat: [[f32; 4]; 4],
}

impl Uniform {
	pub fn new(mat: Mat4) -> Self {
		Self {
			mat: mat.to_cols_array_2d(),
		}
	}
}
