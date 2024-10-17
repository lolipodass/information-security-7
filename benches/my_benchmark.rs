extern crate criterion;
use criterion::{ black_box, criterion_group, criterion_main, Criterion };
use primeculator::modules::{
    substitution_ciphers::{ caesars, trisemus },
    transposition_ciphers::{ double_permutation_encrypt, route_permutation_encrypt },
};

fn benchmark_caesars(c: &mut Criterion) {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let shift = 3;

    let sizes = vec![10, 100, 1000, 10000, 100000];

    let mut group = c.benchmark_group("caesars");
    for &size in &sizes {
        let text = "A".repeat(size);
        group.bench_function(format!("caesars_{}", size), |b| {
            b.iter(|| {
                let result = caesars(
                    black_box(text.clone()),
                    black_box(alphabet),
                    black_box(shift)
                );
                black_box(result);
            })
        });
    }
    group.finish();
}
fn benchmark_trisemus(c: &mut Criterion) {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let shift = 4;

    let sizes = vec![10, 100, 1000, 10000, 100000];

    let mut group = c.benchmark_group("trisemus");
    for &size in &sizes {
        let text = "A".repeat(size);
        group.bench_function(format!("trisemus_{}", size), |b| {
            b.iter(|| {
                let result = trisemus(
                    black_box(text.clone()),
                    black_box(alphabet.to_string()),
                    black_box("enigma"),
                    black_box(shift)
                );
                black_box(result);
            })
        });
    }
    group.finish();
}

fn benchmark_route_permutation(c: &mut Criterion) {
    let sizes = vec![10, 100, 1000, 10000, 100000];

    let mut group = c.benchmark_group("route_permutation");
    for &size in &sizes {
        group.bench_function(format!("route_permutation_{}", size), |b| {
            let text = "A".repeat(size);
            b.iter(|| {
                let result = route_permutation_encrypt(black_box(text.clone()));
                black_box(result);
            })
        });
    }
    group.finish();
}

fn benchmark_double_permutation(c: &mut Criterion) {
    let sizes = vec![10, 100, 1000, 10000, 100000];

    let mut group = c.benchmark_group("double_permutation");
    for &size in &sizes {
        group.bench_function(format!("double_permutation_{}", size), |b| {
            let text = "A".repeat(size);
            b.iter(|| {
                let result = double_permutation_encrypt(
                    black_box(text.clone()),
                    black_box("1320".to_string()),
                    black_box("423051".to_string())
                );
                black_box(result);
            })
        });
    }
    group.finish();
}
criterion_group!(
    benches,
    benchmark_caesars,
    benchmark_trisemus,
    benchmark_route_permutation,
    benchmark_double_permutation
);
criterion_main!(benches);
