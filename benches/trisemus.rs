use criterion::{ black_box, Criterion };
use primeculator::modules::substitution_ciphers::trisemus;

pub fn benchmark_trisemus(c: &mut Criterion) {
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
