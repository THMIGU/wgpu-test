use std::time::Duration;

pub struct FPS {
	fps_timer: Duration,
	fps_frames: u32,
	display_fps: f64,
}

impl FPS {
	pub fn new() -> Self {
		Self {
			fps_timer: Duration::ZERO,
			fps_frames: 0,
			display_fps: 0_f64,
		}
	}

	pub fn fps(&mut self, dt: Duration) -> f64 {
		self.fps_timer += dt;
		self.fps_frames += 1;

		if self.fps_timer > Duration::from_secs(1) {
			self.display_fps = self.fps_frames as f64 / self.fps_timer.as_secs_f64();

			self.fps_timer = Duration::ZERO;
			self.fps_frames = 0;
		}

		self.display_fps
	}
}
