use criterion::{ black_box, Criterion };
use primeculator::modules::transposition_ciphers::route_permutation_encrypt;

pub fn benchmark_route_permutation(c: &mut Criterion) {
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
