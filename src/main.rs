#![windows_subsystem = "windows"]

mod fps;

use sdl3::{
	event::Event,
	pixels::Color,
	sys::render::{SDL_RendererLogicalPresentation, SDL_SetRenderVSync},
};
use std::time::{Duration, Instant};

use crate::fps::FPS;

const TICK_RATE: f64 = 60_f64;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const GAME_WIDTH: u32 = 320;
const GAME_HEIGHT: u32 = 180;

fn main() {
	let sdl_context = sdl3::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem
		.window("wgpu-test", WINDOW_WIDTH, WINDOW_HEIGHT)
		.position_centered()
		.resizable()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas();
	unsafe {
		SDL_SetRenderVSync(canvas.raw(), 1);
	}

	canvas
		.set_logical_size(GAME_WIDTH, GAME_HEIGHT, SDL_RendererLogicalPresentation::LETTERBOX)
		.unwrap();

	let mut event_pump = sdl_context
		.event_pump()
		.unwrap();

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
			accumulator -= tick_time;
		}

		let display_fps = fps.fps(frame_duration);

		canvas
			.window_mut()
			.set_title(&format!("wgpu-test | {:.0} FPS", display_fps))
			.unwrap();

		canvas.set_draw_color(Color::BLACK);
		canvas.clear();

		canvas.present();
	}
}
