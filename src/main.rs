// #![windows_subsystem = "windows"]

mod entity;
mod fps;
mod gfx;
mod scene;

use glam::{Quat, Vec3};
use sdl3::{event::Event, keyboard::Scancode};
use std::time::{Duration, Instant};

use crate::{
	entity::Entity,
	fps::FPS,
	gfx::{
		camera::Camera,
		light::{LightType, LightUniform},
		material::MaterialProperties,
		pipeline::Pipeline,
		transform::{self, Transform},
	},
};

const TICK_RATE: f64 = 120_f64;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

const FOV: f32 = 70_f32.to_radians();

fn main() {
	let sdl_context = sdl3::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let mut window = video_subsystem
		.window("wgpu-test", WINDOW_WIDTH, WINDOW_HEIGHT)
		.position_centered()
		.build()
		.unwrap();

	let mut event_pump = sdl_context
		.event_pump()
		.unwrap();

	let aspect = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;
	let mut camera = Camera::new(FOV, aspect, 0.1, 1000_f32);
	camera.position = Vec3::new(0_f32, 5_f32, -6_f32);

	let mut pipeline = Pipeline::new(&window, true);
	pipeline.update_camera(&camera);

	let skybox_texture = pipeline.load_material(
		"assets/textures/skybox.png",
		MaterialProperties {
			lit: 0,
			shininess: 4_f32,
			specular: 0_f32,
		},
	);
	let asphalt_texture = pipeline.load_material(
		"assets/textures/asphalt.png",
		MaterialProperties {
			lit: 1,
			shininess: 4_f32,
			specular: 0.01_f32,
		},
	);
	let car_texture = pipeline.load_material(
		"assets/textures/car2.png",
		MaterialProperties {
			lit: 1,
			shininess: 64_f32,
			specular: 0.05_f32,
		},
	);

	let skybox_mesh = pipeline.load_obj("assets/models/skybox.obj");
	let asphalt_mesh = pipeline.load_obj("assets/models/plane.obj");
	let car_body_mesh = pipeline.load_obj("assets/models/car/body.obj");
	let car_tire_mesh = pipeline.load_obj("assets/models/car/tire.obj");

	let skybox_model = pipeline.create_model(
		skybox_mesh.clone(),
		Transform::new(Vec3::ZERO, Quat::IDENTITY, Vec3::splat(100_f32)),
		skybox_texture.clone(),
	);
	let asphalt_model = pipeline.create_model(
		asphalt_mesh.clone(),
		Transform::new(Vec3::ZERO, Quat::IDENTITY, Vec3::splat(2_f32)),
		asphalt_texture.clone(),
	);

	let car_entity = Entity::new(transform::IDENTITY);

	let car_body_model = pipeline.create_model(
		car_body_mesh.clone(),
		Transform::new(Vec3::new(0_f32, 0.88, 0.15), Quat::IDENTITY, Vec3::ONE),
		car_texture.clone(),
	);
	let car_tire_fl_model = pipeline.create_model(
		car_tire_mesh.clone(),
		Transform::new(Vec3::new(0.98, 0.42, 1.84), Quat::IDENTITY, Vec3::ONE),
		car_texture.clone(),
	);
	let car_tire_fr_model = pipeline.create_model(
		car_tire_mesh.clone(),
		Transform::new(Vec3::new(-0.98, 0.42, 1.84), Quat::IDENTITY, Vec3::ONE),
		car_texture.clone(),
	);
	let car_tire_bl_model = pipeline.create_model(
		car_tire_mesh.clone(),
		Transform::new(Vec3::new(0.98, 0.42, -1.75), Quat::IDENTITY, Vec3::ONE),
		car_texture.clone(),
	);
	let car_tire_br_model = pipeline.create_model(
		car_tire_mesh.clone(),
		Transform::new(Vec3::new(-0.98, 0.42, -1.75), Quat::IDENTITY, Vec3::ONE),
		car_texture.clone(),
	);

	let sun = LightUniform::new(
		Vec3::new(-1.2, -0.5, 1_f32).normalize(),
		Vec3::new(1_f32, 0.85, 0.54),
		2_f32,
	);

	let mut scene = scene::EMPTY;

	let car_entity_handle = scene.add_entity(car_entity);
	let skybox_handle = scene.add_model(skybox_model);

	scene.add_light(LightType::DirectionalLight(sun));
	scene.add_model(asphalt_model);

	let mut car_models = vec![
		car_body_model,
		car_tire_fl_model,
		car_tire_fr_model,
		car_tire_bl_model,
		car_tire_br_model,
	];

	for model in &mut car_models {
		model.entity_handle = Some(car_entity_handle);
	}
	let handles = scene.add_models(car_models);

	let [
		car_body_handle,
		car_tire_fl_handle,
		car_tire_fr_handle,
		car_tire_bl_handle,
		car_tire_br_handle,
	] = handles.as_slice()
	else {
		panic!("Could not unpack handles!")
	};

	let mut speed = 0_f32;
	let mut spin = 0_f32;
	let mut wheel_steer = Quat::IDENTITY;

	let mut last_frame = Instant::now();
	let mut accumulator = Duration::new(0, 0);
	let tick_time = Duration::from_secs_f64(1_f64 / TICK_RATE);

	let mut fps = FPS::new();

	'running: loop {
		let now = Instant::now();
		let frame_duration = now.duration_since(last_frame);
		accumulator += frame_duration;
		last_frame = now;

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {
					..
				} => break 'running,
				_ => {}
			}
		}

		let tps_scale = 60_f32 / TICK_RATE as f32;

		while accumulator >= tick_time {
			let keyboard = event_pump.keyboard_state();

			let mut angle = 0_f32;
			let mut acceleration = 0_f32;

			if keyboard.is_scancode_pressed(Scancode::W) {
				acceleration += 0.01_f32 * tps_scale;
			}
			if keyboard.is_scancode_pressed(Scancode::S) {
				acceleration -= 0.01_f32 * tps_scale;
			}

			speed += acceleration;

			if keyboard.is_scancode_pressed(Scancode::A) {
				angle += 2.3 * (speed / 0.4);
			}
			if keyboard.is_scancode_pressed(Scancode::D) {
				angle -= 2.3 * (speed / 0.4);
			}

			let car_body = &mut scene.models[*car_body_handle];
			let target = Quat::from_axis_angle(Vec3::Z, (angle * 3.0 * tps_scale).to_radians());
			car_body.transform.rotation = car_body
				.transform
				.rotation
				.slerp(target, 0.1 * tps_scale);

			spin += (speed / 0.4) * 40_f32 * tps_scale;

			if spin > 360.0 {
				spin -= 360.0;
			} else if spin < -360.0 {
				spin += 360.0;
			}

			let target_steer =
				Quat::from_axis_angle(Vec3::Y, (angle * 15.0 * tps_scale).to_radians());

			wheel_steer = wheel_steer.slerp(target_steer, 0.1 * tps_scale);

			let wheel_roll = Quat::from_axis_angle(Vec3::X, spin.to_radians());
			let wheel_rotation = wheel_steer * wheel_roll;

			scene.models[*car_tire_fl_handle]
				.transform
				.rotation = wheel_rotation;
			scene.models[*car_tire_fr_handle]
				.transform
				.rotation = wheel_rotation;

			scene.models[*car_tire_bl_handle]
				.transform
				.rotation = wheel_roll;
			scene.models[*car_tire_br_handle]
				.transform
				.rotation = wheel_roll;

			let friction = 0.975_f32.powf(tps_scale);

			let angle_delta = Quat::from_axis_angle(Vec3::Y, angle.to_radians() * tps_scale);

			let car_entity = &mut scene.entities[car_entity_handle];

			car_entity.transform.rotation *= angle_delta;

			let car_rotation = car_entity
				.transform
				.rotation
				.normalize();
			let forward = car_rotation * Vec3::Z;

			car_entity.transform.position += forward * speed * tps_scale;

			let target = car_entity.transform.position + car_rotation * Vec3::new(0.0, 5.0, -6.0);

			camera.position = (camera
				.position
				.lerp(target, 0.05 * tps_scale)
				- car_entity.transform.position)
				.normalize() * 61_f32.sqrt()
				+ car_entity.transform.position;

			let skybox = &mut scene.models[skybox_handle];
			skybox.transform.position = camera.position;

			camera.rotation =
				Quat::look_at_rh(camera.position, car_entity.transform.position, Vec3::Y).inverse();

			camera.fov = (FOV.to_degrees() + (speed.abs() / 0.4) * 10.0 * tps_scale).to_radians();

			pipeline.update_camera(&camera);

			speed *= friction;

			accumulator -= tick_time;
		}

		let display_fps = fps.fps(frame_duration);

		window
			.set_title(&format!(
				"wgpu-test | {:.0} FPS | x: {:.2} y: {:.2} z: {:.2}",
				display_fps, camera.position.x, camera.position.y, camera.position.z
			))
			.unwrap();

		pipeline.render_scene(&scene);
	}
}
