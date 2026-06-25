//! Performance benchmarks for iridium-units.
//!
//! Run with: `cargo bench`

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use iridium_units::prelude::*;
use iridium_units::quantity::{batch_convert, conversion_factor};

/// Benchmark individual quantity conversions vs batch conversion
fn bench_batch_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_conversion");

    for size in [100, 1000, 10000].iter() {
        let values: Vec<f64> = (0..*size).map(|i| i as f64).collect();

        // Benchmark: Create quantities and convert individually
        group.bench_with_input(BenchmarkId::new("individual", size), size, |b, _| {
            b.iter(|| {
                let converted: Vec<f64> = values
                    .iter()
                    .map(|v| {
                        let q = *v * KM;
                        q.to(M).unwrap().value()
                    })
                    .collect();
                black_box(converted)
            })
        });

        // Benchmark: Use batch conversion
        group.bench_with_input(BenchmarkId::new("batch", size), size, |b, _| {
            b.iter(|| {
                let converted = batch_convert(&values, KM, M).unwrap();
                black_box(converted)
            })
        });

        // Benchmark: Manual factor application
        group.bench_with_input(BenchmarkId::new("manual_factor", size), size, |b, _| {
            let factor = conversion_factor(KM, M).unwrap();
            b.iter(|| {
                let converted: Vec<f64> = values.iter().map(|v| v * factor).collect();
                black_box(converted)
            })
        });
    }

    group.finish();
}

/// Benchmark reference vs value arithmetic operations
fn bench_arithmetic_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("arithmetic");

    let q1 = 100.0 * M;
    let q2 = 50.0 * KM;

    // Addition with references (optimized - no clone)
    group.bench_function("ref_addition", |b| {
        b.iter(|| {
            let result = &q1 + &q2;
            black_box(result)
        })
    });

    // Addition with values (consumes, requires clone to repeat)
    group.bench_function("value_addition", |b| {
        b.iter(|| {
            let a = q1.clone();
            let b = q2.clone();
            let result = a + b;
            black_box(result)
        })
    });

    // Multiplication with references
    group.bench_function("ref_multiplication", |b| {
        b.iter(|| {
            let result = &q1 * &q2;
            black_box(result)
        })
    });

    // Multiplication with values
    group.bench_function("value_multiplication", |b| {
        b.iter(|| {
            let a = q1.clone();
            let b = q2.clone();
            let result = a * b;
            black_box(result)
        })
    });

    group.finish();
}

/// Benchmark unit creation patterns
fn bench_unit_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("unit_operations");

    // Creating quantities
    group.bench_function("quantity_creation", |b| {
        b.iter(|| {
            let q = 42.0 * M;
            black_box(q)
        })
    });

    // Unit division
    group.bench_function("unit_division", |b| {
        b.iter(|| {
            let u = M / S;
            black_box(u)
        })
    });

    // Composite unit creation
    group.bench_function("composite_unit", |b| {
        b.iter(|| {
            let u = KG * M / (S * S);
            black_box(u)
        })
    });

    // Conversion factor calculation
    group.bench_function("conversion_factor", |b| {
        b.iter(|| {
            let factor = conversion_factor(KM, M).unwrap();
            black_box(factor)
        })
    });

    group.finish();
}

/// Benchmark equivalency conversions
fn bench_equivalencies(c: &mut Criterion) {
    use iridium_units::equivalencies::spectral;

    let mut group = c.benchmark_group("equivalencies");

    let wavelength = 500.0 * NM;

    // Spectral conversion
    group.bench_function("spectral_wavelength_to_freq", |b| {
        b.iter(|| {
            let freq = wavelength.to_equiv(HZ, spectral()).unwrap();
            black_box(freq)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_batch_conversion,
    bench_arithmetic_operations,
    bench_unit_operations,
    bench_equivalencies,
);
criterion_main!(benches);
