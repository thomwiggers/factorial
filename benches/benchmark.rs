use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use factorial::Factorial;
use num_bigint::*;

// To test the performance of the new implementation use
// cargo bench --bench benchmark

#[inline(always)]
fn naive_factorial(n: &BigUint) -> BigUint {
    let mut acc = BigUint::from(1_usize);
    let mut i = BigUint::from(2_usize);
    while &i <= &n {
        acc *= i.clone();
        i += BigUint::from(1_usize);
    }
    acc
}

fn bench_factorial(c: &mut Criterion) {
    let mut group = c.benchmark_group("Factorial");
    for x in [
        5_usize, 10, 20, 50, 100, 200, 500, 1000, 2000, 4000, 8000, 16000,
    ] {
        group.bench_with_input(BenchmarkId::new("Naive", x), &x, 
            |b, x| b.iter(|| naive_factorial(&BigUint::from(*x))));
        group.bench_with_input(BenchmarkId::new("Prime swing", x), &x, 
            |b, x| b.iter(|| BigUint::from(*x).factorial()));
    }
}

criterion_group!(benches, bench_factorial);
criterion_main!(benches);
