use std::collections::HashMap;

use glam::{Quat, Vec2, Vec3};

use crate::gfx::{
	light::{LightType, LightUniform},
	model::Model,
	pipeline::Pipeline,
	transform::Transform,
	vertex::Vertex,
};

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
