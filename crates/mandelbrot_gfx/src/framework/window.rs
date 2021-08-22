use cgmath::{Deg, Euler, Quaternion, Vector3};

use winit::{
	event::{ElementState, Event, KeyboardInput, MouseScrollDelta, VirtualKeyCode, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
};

use super::color::VectorColor;
use super::renderer::Renderer;

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

	pub async fn run(mut self, mut renderer: Renderer) {
		let this_window_id = self.window.id();
		self.event_loop
			.run(move |event, _, control_flow| match event {
				Event::WindowEvent {
					ref event,
					window_id,
				} if window_id == this_window_id => {
					if !renderer.input(event) {
						// UPDATED!
						match event {
							WindowEvent::CloseRequested
							| WindowEvent::KeyboardInput {
								input:
									KeyboardInput {
										state: ElementState::Pressed,
										virtual_keycode: Some(VirtualKeyCode::Escape),
										..
									},
								..
							} => *control_flow = ControlFlow::Exit,
							WindowEvent::Resized(physical_size) => {
								renderer.resize(*physical_size);
							}
							WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
								renderer.resize(**new_inner_size);
							}
							WindowEvent::MouseWheel { delta, .. } => {
								let mut color: Vector3<f64> =
									VectorColor(renderer.clear_color.unwrap_or_default()).into();

								let scrolldir: f64 = match delta {
									MouseScrollDelta::LineDelta(_, b) if *b > 0.0 => 1.0,
									MouseScrollDelta::LineDelta(_, b) if *b <= 0.0 => -1.0,
									_ => 1.0,
								};
								// let a: f64 = delta.into();
								let rotation_amount: f64 = 10.0 * scrolldir;

								let rotation = Quaternion::from(Euler::new(
									Deg(rotation_amount),
									Deg(rotation_amount),
									Deg(rotation_amount),
								));

								color = rotation * color;
								let next_color: VectorColor = color.into();

								renderer.change_clear_color(next_color.0);
								renderer.redraw(); // Make this automatic, maybe with an attribute?
								println!("{:?}, {:?}", delta, next_color)
							}
							_ => {}
						}
					}
				}
				Event::RedrawRequested(_) => renderer.redraw(),
				_ => {}
			});
	}
}
