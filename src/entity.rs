use crate::gfx::transform::Transform;

pub struct Entity {
	pub transform: Transform,
}

impl Entity {
	pub fn new(transform: Transform) -> Self {
		Self {
			transform,
		}
	}
}
