use std::collections::HashMap;

use glam::{Quat, Vec2, Vec3};

use crate::gfx::{
	light::{LightType, LightUniform},
	model::Model,
	renderer::Renderer,
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

	pub fn demo(renderer: &Renderer) -> Self {
		let car1_mesh = renderer.load_obj("assets/models/car1.obj");
		let car1_texture = renderer.create_texture("assets/textures/car1.png");
		let car1_model = renderer.create_model(
			car1_mesh,
			Transform::new(Vec3::new(-2.5, 0_f32, 0_f32), Quat::IDENTITY, Vec3::ONE),
			car1_texture,
		);

		let car2_mesh = renderer.load_obj("assets/models/car2_sep.obj");
		let car2_texture = renderer.create_texture("assets/textures/car2.png");
		let car2_model = renderer.create_model(
			car2_mesh,
			Transform::new(Vec3::new(2.5, 0_f32, 0_f32), Quat::IDENTITY, Vec3::ONE),
			car2_texture,
		);

		let cube_mesh = renderer.load_obj("assets/models/cube.obj");
		let cube_texture = renderer.create_texture("assets/textures/cube.png");
		let cube_model = renderer.create_model(
			cube_mesh,
			Transform::new(Vec3::ZERO, Quat::IDENTITY, Vec3::splat(100_f32)),
			cube_texture,
		);

		let plane_vertices = vec![
			Vertex::new(Vec3::new(-1.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0), Vec2::new(0.0, 0.0)),
			Vertex::new(Vec3::new(1.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0), Vec2::new(1.0, 0.0)),
			Vertex::new(Vec3::new(1.0, 0.0, 1.0), Vec3::new(0.0, 1.0, 0.0), Vec2::new(1.0, 1.0)),
			Vertex::new(Vec3::new(-1.0, 0.0, 1.0), Vec3::new(0.0, 1.0, 0.0), Vec2::new(0.0, 1.0)),
		];
		let plane_indices: Vec<u32> = vec![0, 2, 1, 0, 3, 2];

		let plane_mesh = renderer.create_mesh(plane_vertices, plane_indices);
		let plane_texture = renderer.create_texture("assets/textures/asphalt.png");
		let plane_model = renderer.create_model(
			plane_mesh,
			Transform::new(Vec3::ZERO, Quat::IDENTITY, Vec3::splat(10_f32)),
			plane_texture,
		);

		let sun = LightUniform::new(
			Vec3::new(-1.2, -0.5, 1_f32).normalize(),
			Vec3::new(1_f32, 0.85, 0.54),
			1_f32,
		);

		Scene::new(
			vec![car1_model, car2_model, cube_model, plane_model],
			vec![LightType::DirectionalLight(sun)],
		)
	}
}
