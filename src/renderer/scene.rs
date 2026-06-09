use crate::renderer::{light::LightType, model::Model};

pub struct Scene {
	pub models: Vec<Model>,
	pub lights: Vec<LightType>,
}

impl Scene {
	pub fn new(models: Vec<Model>, lights: Vec<LightType>) -> Self {
		Self {
			models,
			lights,
		}
	}
}
