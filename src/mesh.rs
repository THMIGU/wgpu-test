use wgpu::{Device, util::DeviceExt};

use crate::vertex::Vertex;

pub struct Mesh {
	pub vertex_buffer: wgpu::Buffer,
	pub index_buffer: wgpu::Buffer,
	pub index_count: u32,
}

impl Mesh {
	pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, device: &Device) -> Self {
		let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: None,
			contents: bytemuck::cast_slice(&vertices),
			usage: wgpu::BufferUsages::VERTEX,
		});
		let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: None,
			contents: bytemuck::cast_slice(&indices),
			usage: wgpu::BufferUsages::INDEX,
		});

		Self {
			vertex_buffer,
			index_buffer,
			index_count: indices.len() as u32,
		}
	}

	pub fn from_obj(path: &str, device: &Device) -> Self {
		let obj = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS).unwrap();
		let (models, _) = obj;

		let mesh = &models[0].mesh;

		let vertices: Vec<Vertex> = mesh
			.positions
			.chunks(3)
			.map(|c| {
				Vertex::new(
					c[0],
					c[1],
					c[2],
					rand::random::<f32>(),
					rand::random::<f32>(),
					rand::random::<f32>(),
				)
			})
			.collect();

		let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: None,
			contents: bytemuck::cast_slice(&vertices),
			usage: wgpu::BufferUsages::VERTEX,
		});
		let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: None,
			contents: bytemuck::cast_slice(&mesh.indices),
			usage: wgpu::BufferUsages::INDEX,
		});

		Self {
			vertex_buffer,
			index_buffer,
			index_count: mesh.indices.len() as u32,
		}
	}
}
