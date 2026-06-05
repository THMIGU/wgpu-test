use glam::{Mat4, Quat, Vec3};

pub struct Camera {
	pub position: Vec3,
	pub rotation: Quat,

	pub fov: f32,
	pub aspect: f32,
	pub near: f32,
	pub far: f32,
}

impl Camera {
	pub fn new(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
		Self {
			position: Vec3::ZERO,
			rotation: Quat::IDENTITY,
			fov,
			aspect,
			near,
			far,
		}
	}

	fn view_matrix(&self) -> Mat4 {
		Mat4::from_rotation_translation(self.rotation, self.position).inverse()
	}

	fn projection_matrix(&self) -> Mat4 {
		Mat4::perspective_rh_gl(self.fov, self.aspect, self.near, self.far)
	}

	pub fn view_proj_matrix(&self) -> Mat4 {
		self.projection_matrix() * self.view_matrix()
	}
}
