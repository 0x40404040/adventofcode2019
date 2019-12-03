use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_01::{total_fuel, total_fuel_recursive, total_fuel_iter};
use std::u32;

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("total fuel", |b| b.iter(|| total_fuel(black_box(u32::MAX))));

	c.bench_function("total fuel recursive", |b| {
		b.iter(|| total_fuel_recursive(black_box(u32::MAX)))
	});

	c.bench_function("total fuel iter", |b| {
		b.iter(|| total_fuel_iter(black_box(u32::MAX)))
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
