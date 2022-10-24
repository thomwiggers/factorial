use criterion::{black_box, criterion_group, criterion_main, Criterion};
use factorial::Factorial;
use num_bigint::*;
use primal_sieve::Sieve;

fn psw_factorial_benchmark(c: &mut Criterion) {
    let sieve = Sieve::new(5_000);
    c.bench_function("factorial", |b| {
        b.iter(|| {
            black_box(5_000_usize)
                .to_biguint()
                .unwrap()
                .psw_factorial(&sieve)
        })
    });
}

fn factorial_benchmark(c: &mut Criterion) {
    c.bench_function("factorial", |b| {
        b.iter(|| black_box(5_000_usize).to_biguint().unwrap().factorial())
    });
}

criterion_group!(benches, psw_factorial_benchmark, factorial_benchmark);
criterion_main!(benches);
