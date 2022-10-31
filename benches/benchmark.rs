use criterion::{black_box, criterion_group, criterion_main, Criterion};
use factorial::Factorial;
use num_bigint::*;

fn psw_factorial_benchmark(c: &mut Criterion) {
    for x in [
        5_usize, 10, 20, 50, 100, 200, 500, 1000, 2000, 4000, 8000, 16000,
    ] {
        let id = format!("Psw factorial of {}", x);
        c.bench_function(&id, |b| {
            b.iter(|| black_box(x).to_biguint().unwrap().factorial())
        });
        let id = format!("Naive factorial of {}", x);
        c.bench_function(&id, |b| {
            b.iter(|| black_box(x).to_biguint().unwrap().naive_factorial())
        });
    }
}

fn factorial_benchmark(c: &mut Criterion) {
    c.bench_function("factorial", |b| {
        b.iter(|| black_box(5_000_usize).to_biguint().unwrap().factorial())
    });
}

criterion_group!(benches, psw_factorial_benchmark, factorial_benchmark);
criterion_main!(benches);
