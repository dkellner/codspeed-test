use codspeed_test::fibonacci;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci(20)", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, fibonacci_benchmark);
criterion_main!(benches);
