use criterion::{ black_box, Criterion };
use primeculator::modules::md5::md5::md5;
pub fn md5_benchmark(c: &mut Criterion) {
    let sizes = vec![10, 100, 1000, 10000, 100000];

    let mut group = c.benchmark_group("md5_hash");
    for &size in &sizes {
        let text = "A".repeat(size).as_bytes().to_vec();
        group.bench_function(format!("md5_hash_{}", size), |b| {
            b.iter(|| {
                let encrypt = md5(&text);
                black_box(encrypt);
            })
        });
    }
    group.finish();
}
