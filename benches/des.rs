use criterion::{ black_box, Criterion };
use primeculator::modules::des::des::{ decrypt_des, encrypt_des };

pub fn des_encrypt_benchmark(c: &mut Criterion) {
    let sizes = vec![10, 100, 1000, 10000, 100000];

    let mut group = c.benchmark_group("des_encrypt_cipher");
    for &size in &sizes {
        group.bench_function(format!("des_encrypt_cipher_{}", size), |b| {
            let text = "A".repeat(size);
            b.iter(|| {
                let result = encrypt_des(
                    &text.as_bytes().to_vec(),
                    &vec![0x1f, 0x5a, 0x1f, 0x5a, 0x1f, 0x5a, 0x1f, 0x5a]
                );
                black_box(result);
            })
        });
    }
    group.finish();
}

pub fn des_decrypt_benchmark(c: &mut Criterion) {
    let sizes = vec![10, 100, 1000, 10000, 100000];

    let mut group = c.benchmark_group("des_decrypt_cipher");
    for &size in &sizes {
        group.bench_function(format!("des_decrypt_cipher_{}", size), |b| {
            let text = "A".repeat(size);
            b.iter(|| {
                let result = decrypt_des(
                    &text.as_bytes().to_vec(),
                    &vec![0x1f, 0x5a, 0x1f, 0x5a, 0x1f, 0x5a, 0x1f, 0x5a]
                );
                black_box(result);
            })
        });
    }
    group.finish();
}
