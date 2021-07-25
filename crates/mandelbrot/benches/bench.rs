extern crate criterion;

use criterion::{criterion_group, criterion_main, Criterion};
use mandelbrot::{plot_mandelbrot_par, plot_mandelbrot_simple};

static WIDTH: u32 = 1024;
static HEIGHT: u32 = 1024;

fn plot_mandelbrot_par_benchmark(c: &mut Criterion) {
	c.bench_function("[Mandelbrot] Parallel", |b| {
		b.iter(|| plot_mandelbrot_par(HEIGHT, WIDTH))
	});
}
fn plot_mandelbrot_simple_benchmark(c: &mut Criterion) {
	c.bench_function("[Mandelbrot] Simple", |b| {
		b.iter(|| plot_mandelbrot_simple(HEIGHT, WIDTH))
	});
}

criterion_group!(
	benches,
	plot_mandelbrot_par_benchmark,
	plot_mandelbrot_simple_benchmark,
);
criterion_main!(benches);
