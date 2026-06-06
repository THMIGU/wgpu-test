use crate::{
	camera::Camera, mesh::Mesh, model::Model, scene::Scene, transform::Transform, uniform::Uniform,
	vertex::Vertex,
};

use glam::Mat4;
use sdl3::video::Window;
use wgpu::{
	rwh::{HasDisplayHandle, HasWindowHandle},
	util::DeviceExt,
};

pub struct Renderer {
	device: wgpu::Device,
	surface: wgpu::Surface<'static>,
	queue: wgpu::Queue,
	config: wgpu::SurfaceConfiguration,
	depth_texture: wgpu::Texture,
	depth_view: wgpu::TextureView,
	render_pipeline: wgpu::RenderPipeline,
	view_uniform_buffer: wgpu::Buffer,
	model_bind_group_layout: wgpu::BindGroupLayout,
	view_bind_group: wgpu::BindGroup,
}

impl Renderer {
	pub fn new(window: &Window) -> Self {
		let size = window.size();

		let instance = wgpu::Instance::default();

		let window_handle = window
			.window_handle()
			.unwrap()
			.as_raw();
		let display_handle = window
			.display_handle()
			.unwrap()
			.as_raw();

		let surface_target = wgpu::SurfaceTargetUnsafe::RawHandle {
			raw_display_handle: Some(display_handle),
			raw_window_handle: window_handle,
		};

		let surface = unsafe {
			instance
				.create_surface_unsafe(surface_target)
				.unwrap()
		};

		let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
			power_preference: wgpu::PowerPreference::HighPerformance,
			compatible_surface: Some(&surface),
			force_fallback_adapter: false,
		}))
		.unwrap();

		let (device, queue) =
			pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).unwrap();

		let caps = surface.get_capabilities(&adapter);
		let surface_format = caps.formats[0];
		let alpha_mode = caps.alpha_modes[0];

		let config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: surface_format,
			width: size.0,
			height: size.1,
			present_mode: wgpu::PresentMode::Fifo,
			alpha_mode: alpha_mode,
			view_formats: vec![],
			desired_maximum_frame_latency: 2,
		};

		surface.configure(&device, &config);

		let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
			label: Some("Depth Texture"),
			size: wgpu::Extent3d {
				width: config.width,
				height: config.height,
				depth_or_array_layers: 1,
			},
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: wgpu::TextureFormat::Depth32Float,
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			view_formats: &[],
		});
		let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

		let view_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("View Uniform Buffer"),
			contents: bytemuck::cast_slice(&[Uniform::new(Mat4::IDENTITY.to_cols_array_2d())]),
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
		});

		let view_bind_group_layout =
			device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
				label: Some("View Uniform Layout"),
				entries: &[wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStages::VERTEX,
					ty: wgpu::BindingType::Buffer {
						ty: wgpu::BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None,
					},
					count: None,
				}],
			});
		let model_bind_group_layout =
			device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
				label: Some("Model Uniform Layout"),
				entries: &[wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStages::VERTEX,
					ty: wgpu::BindingType::Buffer {
						ty: wgpu::BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None,
					},
					count: None,
				}],
			});

		let view_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: Some("View Uniform Bind Group"),
			layout: &view_bind_group_layout,
			entries: &[wgpu::BindGroupEntry {
				binding: 0,
				resource: view_uniform_buffer.as_entire_binding(),
			}],
		});

		let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
			label: Some("Triangle Shader"),
			source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
		});

		let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: Some("Pipeline Layout"),
			bind_group_layouts: &[Some(&view_bind_group_layout), Some(&model_bind_group_layout)],
			immediate_size: 0,
		});

		let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some("Triangle Pipeline"),
			layout: Some(&pipeline_layout),
			vertex: wgpu::VertexState {
				module: &shader,
				entry_point: Some("vs_main"),
				buffers: &[Vertex::desc()],
				compilation_options: Default::default(),
			},
			fragment: Some(wgpu::FragmentState {
				module: &shader,
				entry_point: Some("fs_main"),
				compilation_options: Default::default(),
				targets: &[Some(wgpu::ColorTargetState {
					format: surface_format,
					blend: Some(wgpu::BlendState::REPLACE),
					write_mask: wgpu::ColorWrites::ALL,
				})],
			}),
			primitive: wgpu::PrimitiveState {
				cull_mode: Some(wgpu::Face::Back),
				..Default::default()
			},
			depth_stencil: Some(wgpu::DepthStencilState {
				format: wgpu::TextureFormat::Depth32Float,
				depth_write_enabled: Some(true),
				depth_compare: Some(wgpu::CompareFunction::Less),
				stencil: Default::default(),
				bias: Default::default(),
			}),
			multisample: Default::default(),
			multiview_mask: None,
			cache: None,
		});

		Self {
			surface,
			device,
			queue,
			config,
			depth_texture,
			depth_view,
			render_pipeline,
			view_uniform_buffer,
			model_bind_group_layout,
			view_bind_group,
		}
	}

	pub fn update_camera(&mut self, camera: &Camera) {
		let uniform = Uniform::new(
			camera
				.view_proj_matrix()
				.to_cols_array_2d(),
		);

		self.queue
			.write_buffer(&self.view_uniform_buffer, 0, bytemuck::cast_slice(&[uniform]));
	}

	pub fn create_mesh(&self, vertices: Vec<Vertex>, indices: Vec<u32>) -> Mesh {
		Mesh::new(vertices, indices, &self.device)
	}

	pub fn create_mesh_from_obj(&self, path: &str) -> Mesh {
		Mesh::from_obj(path, &self.device)
	}

	pub fn create_model(&self, mesh: Mesh, transform: Transform) -> Model {
		Model::new(mesh, transform, &self.device, &self.model_bind_group_layout)
	}

	pub fn render_scene(&mut self, scene: &Scene) {
		let frame = match self
			.surface
			.get_current_texture()
		{
			wgpu::CurrentSurfaceTexture::Success(frame)
			| wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
			_ => panic!("Surface error!"),
		};

		let view = frame
			.texture
			.create_view(&wgpu::TextureViewDescriptor::default());
		let mut encoder = self
			.device
			.create_command_encoder(&wgpu::CommandEncoderDescriptor {
				label: Some("Render Encoder"),
			});

		{
			let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: Some("Render Pass"),
				color_attachments: &[Some(wgpu::RenderPassColorAttachment {
					view: &view,
					resolve_target: None,
					depth_slice: None,
					ops: wgpu::Operations {
						load: wgpu::LoadOp::Clear(wgpu::Color {
							r: 0_f64,
							g: 0_f64,
							b: 0_f64,
							a: 1_f64,
						}),
						store: wgpu::StoreOp::Store,
					},
				})],
				depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
					view: &self.depth_view,
					depth_ops: Some(wgpu::Operations {
						load: wgpu::LoadOp::Clear(1_f32),
						store: wgpu::StoreOp::Store,
					}),
					stencil_ops: None,
				}),
				occlusion_query_set: None,
				timestamp_writes: None,
				multiview_mask: None,
			});
			render_pass.set_pipeline(&self.render_pipeline);

			render_pass.set_bind_group(0, &self.view_bind_group, &[]);

			for model in &scene.models {
				render_pass.set_bind_group(1, &model.model_bind_group, &[]);

				render_pass.set_vertex_buffer(
					0,
					model
						.mesh
						.vertex_buffer
						.slice(..),
				);
				render_pass.set_index_buffer(
					model
						.mesh
						.index_buffer
						.slice(..),
					wgpu::IndexFormat::Uint32,
				);

				let uniform = Uniform::new(
					model
						.transform
						.matrix()
						.to_cols_array_2d(),
				);

				self.queue.write_buffer(
					&model.model_uniform_buffer,
					0,
					bytemuck::cast_slice(&[uniform]),
				);

				render_pass.draw_indexed(0..model.mesh.index_count, 0, 0..1);
			}
		}

		self.queue
			.submit(Some(encoder.finish()));

		frame.present();
	}
}
