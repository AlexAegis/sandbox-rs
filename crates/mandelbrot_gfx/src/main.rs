use mandelbrot_gfx::framework::renderer::Renderer;
use mandelbrot_gfx::framework::window::Window;

#[tokio::main]
pub async fn main() {
	let window = Window::new();
	let engine = Renderer::new(&window.window).await;
	engine.print_env();

	window.run(engine).await;
}
