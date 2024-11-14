use criterion::{ black_box, Criterion };
use primeculator::modules::el_gamal::ElGamal;
pub fn el_gamal_benchmark(c: &mut Criterion) {
    let sizes = vec![10, 100, 1000, 10000, 100000];

    let mut group = c.benchmark_group("el_gamal_encrypt_cipher");
    let el_gamal = ElGamal::new(100);
    for &size in &sizes {
        let text = "A".repeat(size).as_bytes().to_vec();
        group.bench_function(format!("el_gamal_encrypt_cipher_{}", size), |b| {
            b.iter(|| {
                let encrypt = el_gamal.encrypt(black_box(&text.clone()));
                black_box(encrypt);
            })
        });
        group.bench_function(format!("el_gamal_decrypt_cipher_{}", size), |b| {
            let encrypt = el_gamal.encrypt(&text.clone());
            b.iter(|| {
                let decrypt = el_gamal.decrypt(black_box(&encrypt.clone()));
                black_box(decrypt);
            })
        });
    }
    group.finish();
}
