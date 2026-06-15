use bytemuck::{Pod, Zeroable};
use image::imageops;
use wgpu::{Device, Queue, util::DeviceExt};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct MaterialProperties {
	pub lit: u32,
}

pub const DEFAULT_PROP: MaterialProperties = MaterialProperties {
	lit: 1,
};

pub struct Material {
	pub texture: wgpu::Texture,
	pub sampler: wgpu::Sampler,
	pub material_storage_buffer: wgpu::Buffer,
	pub material_bind_group: wgpu::BindGroup,
}

impl Material {
	pub fn new(
		path: &str,
		properties: MaterialProperties,
		device: &Device,
		queue: &Queue,
		material_bind_group_layout: &wgpu::BindGroupLayout,
	) -> Self {
		let img = image::open(path)
			.unwrap()
			.to_rgba8();
		let img = imageops::flip_vertical(&img);

		let size = wgpu::Extent3d {
			width: img.width(),
			height: img.height(),
			depth_or_array_layers: 1,
		};

		let texture = device.create_texture(&wgpu::TextureDescriptor {
			label: Some("Texture"),
			size,
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: wgpu::TextureFormat::Rgba8UnormSrgb,
			usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
			view_formats: &[],
		});
		let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

		let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
			label: Some("Texture Sampler"),
			address_mode_u: wgpu::AddressMode::Repeat,
			address_mode_v: wgpu::AddressMode::Repeat,
			address_mode_w: wgpu::AddressMode::Repeat,
			mag_filter: wgpu::FilterMode::Nearest,
			min_filter: wgpu::FilterMode::Nearest,
			mipmap_filter: wgpu::MipmapFilterMode::Nearest,
			..Default::default()
		});

		queue.write_texture(
			texture.as_image_copy(),
			&img,
			wgpu::TexelCopyBufferLayout {
				offset: 0,
				bytes_per_row: Some(4 * img.width()),
				rows_per_image: Some(img.height()),
			},
			size,
		);

		let material_storage_buffer =
			device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
				label: Some("Material Storage Buffer"),
				contents: bytemuck::cast_slice(&[properties]),
				usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
			});

		let material_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: Some("Texture Bind Group"),
			layout: material_bind_group_layout,
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: wgpu::BindingResource::TextureView(&view),
				},
				wgpu::BindGroupEntry {
					binding: 1,
					resource: wgpu::BindingResource::Sampler(&sampler),
				},
				wgpu::BindGroupEntry {
					binding: 2,
					resource: material_storage_buffer.as_entire_binding(),
				},
			],
		});

		Self {
			texture,
			sampler,
			material_storage_buffer,
			material_bind_group,
		}
	}
}
