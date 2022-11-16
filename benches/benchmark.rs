use std::time::Duration;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, PlotConfiguration, AxisScale};
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
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    group.warm_up_time(Duration::new(1, 0));
    group.measurement_time(Duration::new(2, 0));
    group.noise_threshold(0.1);
    for x in [1usize, 2, 7, 10, 25, 50, 75, 100, 120, 150, 200, 250, 500, 600, 690, 750, 1000] {
        group.bench_with_input(BenchmarkId::new("Naive", x), &x, |b, x| {
            b.iter(|| naive_factorial(&BigUint::from(*x)))
        });
        group.bench_with_input(BenchmarkId::new("Prime swing", x), &x, |b, x| {
            b.iter(|| BigUint::from(*x).factorial())
        });
    }
    group.measurement_time(Duration::new(10, 0));
    group.sample_size(30);
    for x in [2500usize, 5000, 7500, 10000, 25000, 50000] {
        group.bench_with_input(BenchmarkId::new("Naive", x), &x, |b, x| {
            b.iter(|| naive_factorial(&BigUint::from(*x)))
        });
        group.bench_with_input(BenchmarkId::new("Prime swing", x), &x, |b, x| {
            b.iter(|| BigUint::from(*x).factorial())
        });
    }
    group.measurement_time(Duration::new(69, 0));
    group.sample_size(10);
    for x in [75000usize, 100000] {
        group.bench_with_input(BenchmarkId::new("Naive", x), &x, |b, x| {
            b.iter(|| naive_factorial(&BigUint::from(*x)))
        });
        group.bench_with_input(BenchmarkId::new("Prime swing", x), &x, |b, x| {
            b.iter(|| BigUint::from(*x).factorial())
        });
    }

    group.finish()
}

criterion_group!(benches, bench_factorial);
criterion_main!(benches);
