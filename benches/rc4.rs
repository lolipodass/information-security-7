use criterion::{ black_box, Criterion };
use primeculator::modules::generators::rc4;

pub fn rc4_benchmark(c: &mut Criterion) {
    let sizes = vec![10, 100, 1000, 10000, 100000];

    let mut group = c.benchmark_group("rc4");
    for &size in &sizes {
        group.bench_function(format!("rc4_encryption_{}", size), |b| {
            let text = "A".repeat(size);
            b.iter(|| {
                let result = rc4(text.clone().into_bytes(), 6, vec![61, 60, 23, 22, 60, 61]);
                black_box(result);
            })
        });
    }
    group.finish();
}
