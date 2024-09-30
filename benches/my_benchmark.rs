extern crate criterion;
use criterion::{ black_box, criterion_group, criterion_main, Criterion };
use primeculator::modules::substitution_ciphers::caesars;

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

criterion_group!(benches, benchmark_caesars);
criterion_main!(benches);
