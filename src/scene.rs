use crate::{
	entity::Entity,
	gfx::{light::LightType, model::Model},
};

pub const EMPTY: Scene = Scene {
	entities: vec![],
	models: vec![],
	lights: vec![],
};

pub struct Scene {
	pub entities: Vec<Entity>,
	pub models: Vec<Model>,
	pub lights: Vec<LightType>,
}

impl Scene {
	pub fn new(entities: Vec<Entity>, models: Vec<Model>, lights: Vec<LightType>) -> Self {
		Self {
			entities,
			models,
			lights,
		}
	}

	pub fn add_entity(&mut self, entity: Entity) -> u32 {
		let handle = self.entities.len();
		self.entities.push(entity);

		handle as u32
	}

	pub fn add_model(&mut self, model: Model) -> u32 {
		let handle = self.models.len();
		self.models.push(model);

		handle as u32
	}

	pub fn add_models(&mut self, models: Vec<Model>) {
		for model in models {
			self.add_model(model);
		}
	}

	pub fn add_light(&mut self, light: LightType) -> u32 {
		let handle = self.lights.len();
		self.lights.push(light);

		handle as u32
	}
}
