
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use minigrep::{fibonacci, fibonacci_new};

pub fn criterion_benchmark(c: &mut Criterion){
    c.bench_function("fib", |b| b.iter(|| fibonacci_new(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
