use mandelbrot::plot_mandelbrot_par;

fn main() {
	let image = plot_mandelbrot_par(1024, 1024);

	image
		.save("target/mandelbrot.png")
		.expect("Failed to write image");
}
