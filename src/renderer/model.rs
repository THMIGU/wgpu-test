use std::sync::Arc;

use glam::Mat4;
use wgpu::{Device, util::DeviceExt};

use crate::renderer::{mesh::Mesh, texture::Texture, transform::Transform, uniform::Uniform};

pub struct Model {
	pub mesh: Mesh,
	pub transform: Transform,
	pub texture: Arc<Texture>,
	pub model_uniform_buffer: wgpu::Buffer,
	pub model_bind_group: wgpu::BindGroup,
}

impl Model {
	pub fn new(
		mesh: Mesh,
		transform: Transform,
		texture: Arc<Texture>,
		device: &Device,
		model_bind_group_layout: &wgpu::BindGroupLayout,
	) -> Self {
		let model_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("Model Uniform Buffer"),
			contents: bytemuck::cast_slice(&[Uniform::new(Mat4::IDENTITY)]),
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
		});

		let model_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: Some("Model Bind Group"),
			layout: &model_bind_group_layout,
			entries: &[wgpu::BindGroupEntry {
				binding: 0,
				resource: model_uniform_buffer.as_entire_binding(),
			}],
		});

		Self {
			mesh,
			transform,
			texture,
			model_uniform_buffer,
			model_bind_group,
		}
	}
}
