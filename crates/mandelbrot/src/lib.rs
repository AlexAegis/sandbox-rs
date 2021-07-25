use std::usize;

use image::{ImageBuffer, Rgb};
use num::Complex;
use rayon::prelude::*;

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

fn create_pixels(height: u32, width: u32) -> Vec<(u32, u32)> {
	let mut pixels = Vec::<(u32, u32)>::new();
	for x in 0..width {
		for y in 0..height {
			pixels.push((x, y));
		}
	}
	pixels
}

/// ~50ms
pub fn plot_mandelbrot_par(height: u32, width: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
	create_pixels(height, width)
		.par_iter()
		.map(|(x, y)| {
			match is_mandelbrot(
				pixel_to_point(
					(height, width),
					(*x, *y),
					(Complex::new(-3., 2.4), Complex::new(-1.2, 0.6)),
				),
				255,
			) {
				Some(i) => (
					*x,
					*y,
					Rgb([
						255 - i,
						num::clamp((i as usize) * 22, 0, 255) as u8,
						num::clamp((i as usize) * 42, 0, 255) as u8,
					]),
				),
				None => (*x, *y, Rgb([12, 12, 12])),
			}
		})
		.collect::<Vec<(u32, u32, Rgb<u8>)>>()
		.into_iter()
		.fold(
			ImageBuffer::from_raw(width, height, vec![0; 3 * height as usize * width as usize])
				.expect("Couldn't create ImageBuffer"),
			|mut acc, (x, y, rgb)| {
				acc.get_pixel_mut(x, y).0.copy_from_slice(&rgb.0);
				acc
			},
		)
}

/// ~391ms
pub fn plot_mandelbrot_simple(height: u32, width: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
	ImageBuffer::from_fn(width, height, |x, y| {
		match is_mandelbrot(
			pixel_to_point(
				(height, width),
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
	})
}
