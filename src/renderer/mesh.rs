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

	pub fn load_obj(path: &str, device: &Device) -> Self {
		let obj = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS).unwrap();
		let (models, _) = obj;

		let mut vertices: Vec<Vertex> = vec![];
		let mut indices: Vec<u32> = vec![];

		for model in models {
			let mesh = &model.mesh;
			let base_vertex = vertices.len() as u32;
			let mut local_vertices = vec![];

			for i in 0..mesh.positions.len() / 3 {
				let px = mesh.positions[i * 3];
				let py = mesh.positions[i * 3 + 1];
				let pz = mesh.positions[i * 3 + 2];

				let nx = mesh.normals[i * 3];
				let ny = mesh.normals[i * 3 + 1];
				let nz = mesh.normals[i * 3 + 2];

				let u = mesh.texcoords[i * 2];
				let v = mesh.texcoords[i * 2 + 1];

				local_vertices.push(Vertex::new(
					Vec3::new(px, py, pz),
					Vec3::new(nx, ny, nz),
					Vec2::new(u, v),
				));
			}

			vertices.extend(local_vertices);

			for &idx in &mesh.indices {
				indices.push(base_vertex + idx);
			}
		}

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
}
