use std::borrow::Cow;

use wgpu::{
	Adapter, Device, Extent3d, Instance, Queue, RenderPipeline, ShaderModule, Surface,
	TextureFormat,
};
use winit::{
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::Window,
};

#[derive(Debug)]
pub struct Engine {
	pub(crate) instance: Instance,
	pub(crate) surface: Surface,
	pub(crate) surface_config: wgpu::SurfaceConfiguration,
	pub(crate) device: Device,
	pub(crate) render_pipeline: RenderPipeline,
	pub(crate) queue: Queue,
}

impl Engine {
	pub async fn new(window: &Window) -> Self {
		let size = window.inner_size();

		let instance = wgpu::Instance::new(wgpu::Backends::all());
		let surface = unsafe { instance.create_surface(window) };
		let adapter = instance
			.request_adapter(&wgpu::RequestAdapterOptions {
				power_preference: wgpu::PowerPreference::default(),
				compatible_surface: Some(&surface),
			})
			.await
			.expect("Failed to find an appropiate adapter");

		// Create the logical device and command queue
		let (device, queue) = adapter
			.request_device(
				&wgpu::DeviceDescriptor {
					label: None,
					features: wgpu::Features::empty(),
					// Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
					limits: wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits()),
				},
				None,
			)
			.await
			.expect("Failed to create device");

		let texture_format = surface.get_preferred_format(&adapter).unwrap();

		let render_pipeline = Engine::create_render_pipeline(&device, texture_format);

		let surface_config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: texture_format,
			width: size.width,
			height: size.height,
			present_mode: wgpu::PresentMode::Mailbox,
		};

		let instance = Self {
			instance,
			surface,
			surface_config,
			device,
			render_pipeline,
			queue,
		};

		instance.reconfigure_surface();

		instance
	}

	fn create_render_pipeline(device: &Device, texture_format: TextureFormat) -> RenderPipeline {
		let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
			label: None,
			source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/shader.wgsl"))),
		});

		let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: None,
			bind_group_layouts: &[],
			push_constant_ranges: &[],
		});

		let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: None,
			layout: Some(&pipeline_layout),
			vertex: wgpu::VertexState {
				module: &shader,
				entry_point: "vs_main",
				buffers: &[],
			},
			fragment: Some(wgpu::FragmentState {
				module: &shader,
				entry_point: "fs_main",
				targets: &[texture_format.into()],
			}),
			primitive: wgpu::PrimitiveState::default(),
			depth_stencil: None,
			multisample: wgpu::MultisampleState::default(),
		});

		render_pipeline
	}

	fn reconfigure_surface(&self) {
		self.surface.configure(&self.device, &self.surface_config);
	}

	pub fn resize_surface(&mut self, height: u32, width: u32) {
		self.surface_config.height = height;
		self.surface_config.width = width;
		self.reconfigure_surface();
	}

	pub fn redraw(&self) {
		let frame = self
			.surface
			.get_current_frame()
			.expect("Failed to acquire next swap chain texture")
			.output;

		let view = frame
			.texture
			.create_view(&wgpu::TextureViewDescriptor::default());

		let mut encoder = self
			.device
			.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
		{
			let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: None,
				color_attachments: &[wgpu::RenderPassColorAttachment {
					view: &view,
					resolve_target: None,
					ops: wgpu::Operations {
						load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
						store: true,
					},
				}],
				depth_stencil_attachment: None,
			});
			rpass.set_pipeline(&self.render_pipeline);
			rpass.draw(0..3, 0..1);
		}
		self.queue.submit(Some(encoder.finish()));
	}

	pub fn print_env(&self) {
		println!(
			"WGPU_BACKEND: {}",
			std::env::var("WGPU_BACKEND").unwrap_or("Not set".to_string())
		);
	}
}
