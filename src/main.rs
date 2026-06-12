// #![windows_subsystem = "windows"]

mod fps;
mod gfx;

use glam::{Quat, Vec3};
use sdl3::{event::Event, keyboard::Scancode};
use std::time::{Duration, Instant};

use crate::{
	fps::FPS,
	gfx::{
		camera::Camera,
		light::{LightType, LightUniform},
		pipeline::Pipeline,
		scene::Scene,
		transform::{self, Transform},
	},
};

const TICK_RATE: f64 = 60_f64;

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

	sdl_context
		.mouse()
		.set_relative_mouse_mode(&window, true);

	let mut event_pump = sdl_context
		.event_pump()
		.unwrap();

	let aspect = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;
	let mut camera = Camera::new(FOV, aspect, 0.1, 1000_f32);
	camera.position = Vec3::new(0_f32, 0_f32, 10_f32);

	let mut pipeline = Pipeline::new(&window, true);
	pipeline.update_camera(&camera);

	let skybox_texture = pipeline.load_texture("assets/textures/skybox.png");
	let asphalt_texture = pipeline.load_texture("assets/textures/asphalt.png");
	let car_texture = pipeline.load_texture("assets/textures/car2.png");

	let skybox_mesh = pipeline.load_obj("assets/models/skybox.obj");
	let asphalt_mesh = pipeline.load_obj("assets/models/plane.obj");
	let car_body_mesh = pipeline.load_obj("assets/models/car/body.obj");
	let car_tire_mesh = pipeline.load_obj("assets/models/car/tire.obj");

	let skybox_model = pipeline.create_model(
		skybox_mesh.clone(),
		Transform::new(Vec3::ZERO, Quat::IDENTITY, Vec3::splat(100_f32)),
		skybox_texture.clone(),
	);
	let asphalt_model =
		pipeline.create_model(asphalt_mesh.clone(), transform::IDENTITY, asphalt_texture.clone());

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
		1_f32,
	);

	let mut scene = Scene::new(
		vec![
			skybox_model,
			asphalt_model,
			car_body_model,
			car_tire_fl_model,
			car_tire_fr_model,
			car_tire_bl_model,
			car_tire_br_model,
		],
		vec![LightType::DirectionalLight(sun)],
	);

	let mut angle = 0_f32;

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

		while accumulator >= tick_time {
			let keyboard = event_pump.keyboard_state();

			let camera_yaw = camera
				.rotation
				.to_euler(glam::EulerRot::YXZ)
				.0;

			let forward = Vec3::new(camera_yaw.sin(), 0_f32, camera_yaw.cos());
			let right = Vec3::new(forward.z, 0_f32, -forward.x);
			let up = Vec3::new(0_f32, 1_f32, 0_f32);
			let speed = 7.5_f32 / 60_f32;

			let mut movement = Vec3::ZERO;

			if keyboard.is_scancode_pressed(Scancode::S) {
				movement += forward;
			}
			if keyboard.is_scancode_pressed(Scancode::W) {
				movement -= forward;
			}
			if keyboard.is_scancode_pressed(Scancode::D) {
				movement += right;
			}
			if keyboard.is_scancode_pressed(Scancode::A) {
				movement -= right;
			}

			if keyboard.is_scancode_pressed(Scancode::Space) {
				movement += up;
			}
			if keyboard.is_scancode_pressed(Scancode::LShift) {
				movement -= up;
			}

			camera.position += movement.normalize_or_zero() * speed;

			let mouse = event_pump.relative_mouse_state();

			let x = mouse.x();
			let y = mouse.y();

			let sensitivity = 0.0025;

			let yaw = Quat::from_rotation_y(-x as f32 * sensitivity);

			let right = camera.rotation * Vec3::X;
			let pitch = Quat::from_axis_angle(right.normalize(), -y as f32 * sensitivity);

			camera.rotation = pitch * camera.rotation;
			camera.rotation = yaw * camera.rotation;

			if keyboard.is_scancode_pressed(Scancode::C) {
				camera.fov = 30_f32.to_radians();
			} else {
				camera.fov = FOV;
			}

			for i in 0..2 {
				let tire = &mut scene.models[3 + i];
				tire.transform.rotation =
					Quat::from_axis_angle(Vec3::Y, ((angle / 400_f32).sin() * 20_f32).to_radians())
						* Quat::from_axis_angle(Vec3::X, angle.to_radians());
			}
			for i in 0..2 {
				let tire = &mut scene.models[5 + i];
				tire.transform.rotation = Quat::from_axis_angle(Vec3::X, angle.to_radians());
			}
			angle += 10_f32;

			pipeline.update_camera(&camera);

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
