use wgpu::{Adapter, Device, Extent3d, Instance, Surface};
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

		let swapchain_format = surface.get_preferred_format(&adapter).unwrap();

		let surface_config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: swapchain_format,
			width: size.width,
			height: size.height,
			present_mode: wgpu::PresentMode::Mailbox,
		};
		let instance = Self {
			instance,
			surface,
			surface_config,
			device,
		};

		instance.reconfigure_surface();

		instance
	}

	pub fn reconfigure_surface(&self) {
		self.surface.configure(&self.device, &self.surface_config);
	}

	pub fn resize_surface(&mut self, height: u32, width: u32) {
		self.surface_config.height = height;
		self.surface_config.width = width;
		self.reconfigure_surface();
	}

	pub fn print_env(&self) {
		println!(
			"WGPU_BACKEND: {}",
			std::env::var("WGPU_BACKEND").unwrap_or("Not set".to_string())
		);
	}
}
