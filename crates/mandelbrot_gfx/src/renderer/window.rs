use winit::{
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
};

use super::engine::Engine;

pub struct Window {
	pub event_loop: EventLoop<()>,
	pub window: winit::window::Window,
}

impl Window {
	pub fn new() -> Self {
		let event_loop = EventLoop::new();
		let window = winit::window::Window::new(&event_loop).unwrap();
		Window { event_loop, window }
	}

	pub async fn run(self, mut engine: Engine) {
		self.event_loop.run(move |e, _, control_flow| match e {
			Event::WindowEvent {
				event: WindowEvent::CloseRequested,
				..
			} => *control_flow = ControlFlow::Exit,
			Event::WindowEvent {
				event: WindowEvent::Resized(size),
				..
			} => engine.resize_surface(size.height, size.width),
			Event::WindowEvent {
				event: WindowEvent::MouseWheel { delta, .. },
				..
			} => println!("{:?}", delta),
			Event::RedrawRequested(_) => engine.redraw(),
			_ => (),
		})
	}
}
