use std::collections::HashMap;

use glam::{Vec2, Vec3};
use wgpu::{Device, util::DeviceExt};

use crate::renderer::vertex::Vertex;

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

	pub fn load_obj(path: &str, device: &Device) -> HashMap<String, Self> {
		let obj = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS).unwrap();
		let (models, _) = obj;

		let mut meshes: HashMap<String, Self> = HashMap::new();

		for model in models {
			let mesh = &model.mesh;

			let vertices: Vec<Vertex> = mesh
				.positions
				.chunks(3)
				.enumerate()
				.map(|(i, c)| {
					let x = mesh.normals[i * 3];
					let y = mesh.normals[i * 3 + 1];
					let z = mesh.normals[i * 3 + 2];

					let u = mesh.texcoords[i * 2];
					let v = mesh.texcoords[i * 2 + 1];

					Vertex::new(Vec3::new(c[0], c[1], c[2]), Vec3::new(x, y, z), Vec2::new(u, v))
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

			meshes.insert(
				model.name,
				Self {
					vertex_buffer,
					index_buffer,
					index_count: mesh.indices.len() as u32,
				},
			);
		}

		meshes
	}
}
