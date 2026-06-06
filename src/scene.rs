use crate::model::Model;

pub struct Scene {
	pub models: Vec<Model>,
}

impl Scene {
	pub fn new(models: Vec<Model>) -> Self {
		Self {
			models,
		}
	}
}
