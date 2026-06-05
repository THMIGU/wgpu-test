use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Uniform {
	uniform: [[f32; 4]; 4],
}

impl Uniform {
	pub fn new(uniform: [[f32; 4]; 4]) -> Self {
		Self {
			uniform,
		}
	}
}
