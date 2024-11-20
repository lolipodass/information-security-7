use criterion::{ black_box, Criterion };
use primeculator::modules::schnorr_signature;

pub fn schnorr_benchmark(c: &mut Criterion) {
    let sizes = vec![10, 100, 1000, 10000, 100000];

    let mut group = c.benchmark_group("schnorr signature");
    let schnorr = schnorr_signature::SchnorrSignature::new(20);

    for &size in &sizes {
        let text = "A".repeat(size).as_bytes().to_vec();

        group.bench_function(format!("schnorr_sign_{}", size), |b| {
            b.iter(|| {
                let sign = schnorr.sign(black_box(&text));
                black_box(sign);
            })
        });
        group.bench_function(format!("schnorr_verify_{}", size), |b| {
            let sign = schnorr.sign(&text);
            b.iter(|| {
                let verify = schnorr.verify(black_box(&text), black_box(sign.clone()));
                black_box(verify);
            })
        });
    }
    group.finish();
}
