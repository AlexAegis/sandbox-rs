use mandelbrot_gfx::renderer::engine::Engine;
use mandelbrot_gfx::renderer::window::Window;

#[tokio::main]
pub async fn main() {
	let window = Window::new();
	let engine = Engine::new(&window.window).await;
	engine.print_env();

	window.run(engine).await;
}
