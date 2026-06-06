use crate::{mesh::Mesh, transform::Transform};

pub struct Model {
	pub mesh: Mesh,
	pub transform: Transform,
}

impl Model {
	pub fn new(mesh: Mesh, transform: Transform) -> Self {
		Self {
			mesh,
			transform,
		}
	}
}
