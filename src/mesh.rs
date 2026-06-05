use std::{fs::File, io::BufReader};

use obj::{Obj, load_obj};
use wgpu::{Device, util::DeviceExt};

use crate::vertex::Vertex;

pub struct Mesh {
	pub vertex_buffer: wgpu::Buffer,
	pub index_buffer: wgpu::Buffer,
	pub index_count: u32,
}

impl Mesh {
	pub fn new(device: &Device, vertices: Vec<Vertex>, indices: Vec<u16>) -> Self {
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

	pub fn from_obj(device: &Device, path: &str) -> Self {
		let input = BufReader::new(File::open(path).unwrap());
		let obj: Obj = load_obj(input).unwrap();

		let vertices: Vec<Vertex> = obj
			.vertices
			.iter()
			.map(|x| Vertex {
				position: x.position,
				color: [1_f32, 1_f32, 1_f32],
			})
			.collect();

		let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: None,
			contents: bytemuck::cast_slice(&vertices),
			usage: wgpu::BufferUsages::VERTEX,
		});
		let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: None,
			contents: bytemuck::cast_slice(&obj.indices),
			usage: wgpu::BufferUsages::INDEX,
		});

		Self {
			vertex_buffer,
			index_buffer,
			index_count: obj.indices.len() as u32,
		}
	}
}
