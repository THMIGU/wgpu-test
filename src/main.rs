#![windows_subsystem = "windows"]

mod fps;

use bytemuck::{Pod, Zeroable};
use sdl3::event::Event;
use std::time::{Duration, Instant};
use wgpu::{
	rwh::{HasDisplayHandle, HasWindowHandle},
	util::DeviceExt,
};

use crate::fps::FPS;

const TICK_RATE: f64 = 60_f64;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Vertex {
	position: [f32; 2],
	color: [f32; 3],
}

impl Vertex {
	fn new(x: f32, y: f32, r: f32, g: f32, b: f32) -> Self {
		Self {
			position: [x, y],
			color: [r, g, b],
		}
	}

	fn desc() -> wgpu::VertexBufferLayout<'static> {
		wgpu::VertexBufferLayout {
			array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Vertex,
			attributes: &[
				wgpu::VertexAttribute {
					offset: 0,
					shader_location: 0,
					format: wgpu::VertexFormat::Float32x2,
				},
				wgpu::VertexAttribute {
					offset: 8,
					shader_location: 1,
					format: wgpu::VertexFormat::Float32x3,
				},
			],
		}
	}
}

fn main() {
	let sdl_context = sdl3::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let mut window = video_subsystem
		.window("wgpu-test", WINDOW_WIDTH, WINDOW_HEIGHT)
		.position_centered()
		.build()
		.unwrap();
	let size = window.size();

	let mut event_pump = sdl_context
		.event_pump()
		.unwrap();

	// wgpu ===================================

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

	let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
		label: Some("Triangle Shader"),
		source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
	});

	let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
		label: Some("Pipeline Layout"),
		bind_group_layouts: &[],
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
		primitive: Default::default(),
		depth_stencil: None,
		multisample: Default::default(),
		multiview_mask: None,
		cache: None,
	});

	let vertices = [
		Vertex::new(-0.5, 0.5, 1_f32, 0_f32, 0_f32),  // top left
		Vertex::new(0.5, 0.5, 0_f32, 1_f32, 0_f32),   // top right
		Vertex::new(-0.5, -0.5, 0_f32, 0_f32, 1_f32), // bottom left
		Vertex::new(0.5, -0.5, 1_f32, 1_f32, 0_f32),  // bottom right
	];
	let indices: [u16; 6] = [0, 1, 2, 1, 3, 2];

	let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
		label: Some("Vertex Buffer"),
		contents: bytemuck::cast_slice(&vertices),
		usage: wgpu::BufferUsages::VERTEX,
	});
	let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
		label: Some("Index Buffer"),
		contents: bytemuck::cast_slice(&indices),
		usage: wgpu::BufferUsages::INDEX,
	});

	// ========================================

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

		window
			.set_title(&format!("wgpu-test | {:.0} FPS", display_fps))
			.unwrap();

		// wgpu ==================================

		let frame = match surface.get_current_texture() {
			wgpu::CurrentSurfaceTexture::Success(frame)
			| wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
			_ => panic!("Surface error!"),
		};

		let view = frame
			.texture
			.create_view(&wgpu::TextureViewDescriptor::default());
		let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
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
				depth_stencil_attachment: None,
				occlusion_query_set: None,
				timestamp_writes: None,
				multiview_mask: None,
			});
			render_pass.set_pipeline(&render_pipeline);

			render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
			render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);

			render_pass.draw_indexed(0..6, 0, 0..1);
		}

		queue.submit(Some(encoder.finish()));

		frame.present();

		// =======================================
	}
}
