use criterion::{ black_box, Criterion };
use primeculator::modules::el_gamal::ElGamal;
pub fn el_gamal_benchmark(c: &mut Criterion) {
    let sizes = vec![10, 100, 1000, 10000, 100000];

    let mut group = c.benchmark_group("el_gamal_encrypt_cipher");
    let el_gamal = ElGamal::new(100);
    for &size in &sizes {
        let text = "A".repeat(size).as_bytes().to_vec();
        group.bench_function(format!("el_gamal_encrypt_{}", size), |b| {
            b.iter(|| {
                let encrypt = el_gamal.encrypt(black_box(&text));
                black_box(encrypt);
            })
        });
        group.bench_function(format!("el_gamal_decrypt_{}", size), |b| {
            let encrypt = el_gamal.encrypt(&text);
            b.iter(|| {
                let decrypt = el_gamal.decrypt(black_box(&encrypt.clone()));
                black_box(decrypt);
            })
        });
        group.bench_function(format!("el_gamal_signature_{}", size), |b| {
            b.iter(|| {
                let sign = el_gamal.sign(black_box(&text));
                black_box(sign);
            })
        });
        group.bench_function(format!("el_gamal_verify_{}", size), |b| {
            let sign = el_gamal.sign(&text);
            b.iter(|| {
                let verify = el_gamal.verify(black_box(&text), black_box(sign.clone()));
                black_box(verify);
            })
        });
    }
    group.finish();
}
