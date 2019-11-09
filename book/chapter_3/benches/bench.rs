extern crate criterion;

use book_chapter_3::fibonacci;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// cargo bench -p book_chapter_3
fn fibonacci_benchmark(c: &mut Criterion) {
	c.bench_function("[Fibonacci]", |b| {
		b.iter(|| fibonacci::fibonacci::<i64>(black_box(92)))
	});
}

fn fibonacci_recursive_benchmark(c: &mut Criterion) {
	c.bench_function("[Fibonacci] Recursive Larger", |b| {
		b.iter(|| fibonacci::fibonacci_recursive::<i64>(black_box(92)))
	});
}

criterion_group!(benches, fibonacci_benchmark, fibonacci_recursive_benchmark);
criterion_main!(benches);
