use criterion::{ black_box, Criterion };
use primeculator::modules::rsa::RSA;
pub fn rsa_benchmark(c: &mut Criterion) {
    let sizes = vec![10, 100, 1000, 10000, 100000];

    let mut group = c.benchmark_group("rsa_encrypt_cipher");
    let rsa = RSA::new(100);
    for &size in &sizes {
        let text = "A".repeat(size).as_bytes().to_vec();
        group.bench_function(format!("rsa_encrypt_cipher_{}", size), |b| {
            b.iter(|| {
                let encrypt = rsa.encrypt(black_box(&text.clone()));
                black_box(encrypt);
            })
        });
        group.bench_function(format!("rsa_decrypt_cipher_{}", size), |b| {
            let encrypt = rsa.encrypt(&text.clone());
            b.iter(|| {
                let decrypt = rsa.decrypt(black_box(&encrypt.clone()));
                black_box(decrypt);
            })
        });
    }
    group.finish();
}
