// #![windows_subsystem = "windows"]

mod camera;
mod fps;
mod light;
mod material;
mod mesh;
mod model;
mod renderer;
mod scene;
mod transform;
mod uniform;
mod vertex;

use glam::{Quat, Vec2, Vec3};
use sdl3::{event::Event, keyboard::Scancode};
use std::time::{Duration, Instant};

use crate::{
	camera::Camera,
	fps::FPS,
	light::{LightType, LightUniform},
	renderer::Renderer,
	scene::Scene,
	transform::Transform,
	vertex::Vertex,
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

	let car1_mesh = renderer.create_mesh_from_obj("assets/models/car1.obj");
	let car1_material = renderer.create_material_from_texture("assets/textures/car1.png", true);
	let car1_model = renderer.create_model(
		car1_mesh,
		Transform::new(Vec3::new(-2.5, 0_f32, 0_f32), Quat::IDENTITY, Vec3::ONE),
		car1_material,
	);

	let car2_mesh = renderer.create_mesh_from_obj("assets/models/car2.obj");
	let car2_material = renderer.create_material_from_texture("assets/textures/car2.png", true);
	let car2_model = renderer.create_model(
		car2_mesh,
		Transform::new(Vec3::new(2.5, 0_f32, 0_f32), Quat::IDENTITY, Vec3::ONE),
		car2_material,
	);

	let cube_mesh = renderer.create_mesh_from_obj("assets/models/cube.obj");
	let cube_material = renderer.create_material_from_texture("assets/textures/cube.png", false);
	let cube_model = renderer.create_model(
		cube_mesh,
		Transform::new(Vec3::ZERO, Quat::IDENTITY, Vec3::splat(100_f32)),
		cube_material,
	);

	let plane_vertices = vec![
		Vertex::new(Vec3::new(-1.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0), Vec2::new(0.0, 0.0)),
		Vertex::new(Vec3::new(1.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0), Vec2::new(1.0, 0.0)),
		Vertex::new(Vec3::new(1.0, 0.0, 1.0), Vec3::new(0.0, 1.0, 0.0), Vec2::new(1.0, 1.0)),
		Vertex::new(Vec3::new(-1.0, 0.0, 1.0), Vec3::new(0.0, 1.0, 0.0), Vec2::new(0.0, 1.0)),
	];

	let plane_indices: Vec<u32> = vec![0, 2, 1, 0, 3, 2];

	let plane_mesh = renderer.create_mesh(plane_vertices, plane_indices);
	let plane_material = renderer.create_material_from_texture("assets/textures/asphalt.png", true);
	let plane_model = renderer.create_model(
		plane_mesh,
		Transform::new(Vec3::ZERO, Quat::IDENTITY, Vec3::splat(10_f32)),
		plane_material,
	);

	let sun = LightUniform::new(Vec3::new(1.2, 0.5, -1_f32).normalize(), Vec3::ONE, 1_f32);
	let mut scene = Scene::new(
		vec![car1_model, car2_model, cube_model, plane_model],
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
			.set_title(&format!("wgpu-test | {:.0} FPS", display_fps))
			.unwrap();

		renderer.render_scene(&scene);
	}
}
