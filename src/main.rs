#![windows_subsystem = "windows"]

mod fps;

use sdl3::event::Event;
use std::time::{Duration, Instant};
use wgpu::rwh::{HasDisplayHandle, HasWindowHandle};

use crate::fps::FPS;

const TICK_RATE: f64 = 60_f64;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

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
			buffers: &[],
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
							r: 1_f64,
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
			render_pass.draw(0..3, 0..1);
		}

		queue.submit(Some(encoder.finish()));

		frame.present();

		// =======================================
	}
}
