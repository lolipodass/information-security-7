use criterion::{ black_box, Criterion };
use primeculator::modules::enigma::enigma_cipher::enigma_cipher;

pub fn enigma_benchmark(c: &mut Criterion) {
    let sizes = vec![10, 100, 1000, 10000, 100000];

    let mut group = c.benchmark_group("enigma_cipher");
    for &size in &sizes {
        group.bench_function(format!("enigma_cipher_{}", size), |b| {
            let text = "A".repeat(size);
            b.iter(|| {
                let result = enigma_cipher(black_box(text.clone()), (0, 0, 0), (1, 0, 1));
                black_box(result);
            })
        });
    }
    group.finish();
}
