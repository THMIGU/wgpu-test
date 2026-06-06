// #![windows_subsystem = "windows"]

mod camera;
mod fps;
mod mesh;
mod model;
mod renderer;
mod scene;
mod transform;
mod uniform;
mod vertex;

use glam::{Quat, Vec3};
use sdl3::{event::Event, keyboard::Scancode};
use std::time::{Duration, Instant};

use crate::{
	camera::Camera, fps::FPS, renderer::Renderer, scene::Scene, transform::Transform,
	vertex::Vertex,
};

const TICK_RATE: f64 = 60_f64;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

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
	let mut camera = Camera::new(FOV, aspect, 0.1, 100_f32);
	camera.position = Vec3::new(0_f32, 0_f32, 10_f32);

	let mut renderer = Renderer::new(&window);
	renderer.update_camera(&camera);

	let bunny_mesh = renderer.create_mesh_from_obj("models/stanford-bunny.obj");
	let bunny_model = renderer
		.create_model(bunny_mesh, Transform::new(Vec3::ZERO, Quat::IDENTITY, Vec3::splat(20_f32)));

	let teapot_mesh = renderer.create_mesh_from_obj("models/teapot.obj");
	let teapot_model = renderer.create_model(
		teapot_mesh,
		Transform::new(Vec3::new(4_f32, 0_f32, 0_f32), Quat::IDENTITY, Vec3::splat(0.75)),
	);

	let plane_vertices = vec![
		Vertex::new(-1_f32, 0_f32, -1_f32, 1_f32, 1_f32, 1_f32),
		Vertex::new(1_f32, 0_f32, -1_f32, 1_f32, 1_f32, 1_f32),
		Vertex::new(1_f32, 0_f32, 1_f32, 1_f32, 1_f32, 1_f32),
		Vertex::new(-1_f32, 0_f32, 1_f32, 1_f32, 1_f32, 1_f32),
	];
	let plane_indices: Vec<u32> = vec![0, 2, 1, 0, 3, 2];

	let plane_mesh = renderer.create_mesh(plane_vertices, plane_indices);
	let plane_model = renderer
		.create_model(plane_mesh, Transform::new(Vec3::ZERO, Quat::IDENTITY, Vec3::splat(10_f32)));

	let mut scene = Scene::new(vec![bunny_model, teapot_model, plane_model]);

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
			let speed = 5_f32 / 60_f32;

			if keyboard.is_scancode_pressed(Scancode::S) {
				camera.position += forward * speed;
			}
			if keyboard.is_scancode_pressed(Scancode::W) {
				camera.position -= forward * speed;
			}
			if keyboard.is_scancode_pressed(Scancode::D) {
				camera.position += right * speed;
			}
			if keyboard.is_scancode_pressed(Scancode::A) {
				camera.position -= right * speed;
			}

			if keyboard.is_scancode_pressed(Scancode::Space) {
				camera.position += up * speed;
			}
			if keyboard.is_scancode_pressed(Scancode::LShift) {
				camera.position -= up * speed;
			}

			let mouse = event_pump.relative_mouse_state();

			let x = mouse.x();
			let y = mouse.y();

			let sensitivity = 0.005;

			let yaw = Quat::from_rotation_y(-x as f32 * sensitivity);

			let right = camera.rotation * Vec3::X;
			let pitch = Quat::from_axis_angle(right.normalize(), -y as f32 * sensitivity);

			camera.rotation = pitch * camera.rotation;
			camera.rotation = yaw * camera.rotation;

			renderer.update_camera(&camera);

			let models = &mut scene.models;

			let bunny_model = &mut models[0];

			angle += 1_f32;
			bunny_model.transform.rotation = Quat::from_axis_angle(Vec3::Y, angle.to_radians());
			bunny_model
				.transform
				.position
				.y = (angle / 50_f32).sin() * 1_f32 + 2_f32;

			let teapot_model = &mut models[1];
			teapot_model
				.transform
				.rotation = Quat::from_axis_angle(Vec3::Y, (-angle).to_radians());
			teapot_model
				.transform
				.position
				.y = (angle / 50_f32).sin() * -1_f32 + 2_f32;

			accumulator -= tick_time;
		}

		let display_fps = fps.fps(frame_duration);

		window
			.set_title(&format!("wgpu-test | {:.0} FPS", display_fps))
			.unwrap();

		renderer.render_scene(&scene);
	}
}
