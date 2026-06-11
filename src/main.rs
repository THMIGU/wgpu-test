// #![windows_subsystem = "windows"]

mod fps;
mod gfx;

use glam::{Quat, Vec3};
use sdl3::{event::Event, keyboard::Scancode};
use std::time::{Duration, Instant};

use crate::{
	fps::FPS,
	gfx::{camera::Camera, renderer::Renderer, scene::Scene},
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

	let mut renderer = Renderer::new(&window, true);
	renderer.update_camera(&camera);

	let mut scene = Scene::demo(&renderer);

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

			angle += 1_f32;
			let car1 = &mut scene.models[0];
			car1.transform.rotation = Quat::from_axis_angle(Vec3::Y, angle.to_radians());

			let cube = &mut scene.models[2];
			cube.transform.position = camera.position;

			renderer.update_camera(&camera);

			accumulator -= tick_time;
		}

		let display_fps = fps.fps(frame_duration);

		window
			.set_title(&format!(
				"wgpu-test | {:.0} FPS | x: {:.2} y: {:.2} z: {:.2}",
				display_fps, camera.position.x, camera.position.y, camera.position.z
			))
			.unwrap();

		renderer.render_scene(&scene);
	}
}
