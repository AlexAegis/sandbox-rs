extern crate criterion;

use book_chapter_3::fibonacci;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// cargo bench -p book_chapter_3
fn fibonacci_recursive_benchmark(c: &mut Criterion) {
	c.bench_function("[Fibonacci] Recursive", |b| {
		b.iter(|| fibonacci::fibonacci_recursive(black_box(20)))
	});
}

fn fibonacci_recursive_generic_benchmark(c: &mut Criterion) {
	c.bench_function("[Fibonacci] Recursive Generic", |b| {
		b.iter(|| fibonacci::fibonacci_recursive_generic::<i32>(black_box(20i32)))
	});
}

fn fibonacci_recursive_generic_larger_benchmark(c: &mut Criterion) {
	c.bench_function("[Fibonacci] Recursive Generic Larger", |b| {
		b.iter(|| fibonacci::fibonacci_recursive_generic::<i64>(black_box(20i64)))
	});
}

criterion_group!(
	benches,
	fibonacci_recursive_benchmark,
	fibonacci_recursive_generic_benchmark,
	fibonacci_recursive_generic_larger_benchmark
);
criterion_main!(benches);
