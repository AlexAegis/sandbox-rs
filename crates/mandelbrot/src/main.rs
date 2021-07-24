use image::ImageBuffer;
use num::Complex;

fn is_mandelbrot(point: Complex<f64>, limit: u8) -> Option<u8> {
	let mut z = Complex::<f64>::new(0., 0.);
	for i in 0..limit {
		if z.norm_sqr() > 4. {
			return Some(i);
		}
		z = z * z + point;
	}
	None
}

fn pixel_to_point(
	render_size: (u32, u32),
	pixel: (u32, u32),
	plane: (Complex<f64>, Complex<f64>),
) -> Complex<f64> {
	let (width, height) = (plane.1.re - plane.0.re, plane.1.im - plane.0.im);
	Complex::new(
		plane.1.re + pixel.0 as f64 * width / render_size.0 as f64,
		plane.1.im + pixel.1 as f64 * height / render_size.1 as f64,
	)
}

fn main() {
	let img = ImageBuffer::from_fn(1024, 1024, |x, y| {
		match is_mandelbrot(
			pixel_to_point(
				(1024, 1024),
				(x, y),
				(Complex::new(-3., 2.4), Complex::new(-1.2, 0.6)),
			),
			255,
		) {
			Some(i) => image::Rgb([
				255 - i,
				num::clamp((i as usize) * 12, 0, 255) as u8,
				num::clamp((i as usize) * 26, 0, 255) as u8,
			]),
			None => image::Rgb([12, 12, 12]),
		}
	});

	img.save("target/mandelbrot.png")
		.expect("Failed to write image");
}
