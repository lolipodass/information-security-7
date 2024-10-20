use criterion::{ black_box, Criterion };
use primeculator::modules::transposition_ciphers::double_permutation_encrypt;

pub fn benchmark_double_permutation(c: &mut Criterion) {
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
