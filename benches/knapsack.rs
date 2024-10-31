use criterion::{ black_box, Criterion };
use primeculator::modules::knapsack_cipher::KnapsackCipher;

pub fn knapsack_benchmark(c: &mut Criterion) {
    let sizes = vec![10, 100, 1000, 10000, 100000];

    let mut group = c.benchmark_group("knapsack_encrypt_cipher");
    let knapsack = KnapsackCipher::new(8);
    for &size in &sizes {
        let text = "A".repeat(size).as_bytes().to_vec();
        group.bench_function(format!("knapsack_encrypt_cipher_{}", size), |b| {
            b.iter(|| {
                let encrypt = knapsack.encrypt(black_box(text.clone()));
                black_box(encrypt);
            })
        });
        group.bench_function(format!("knapsack_decrypt_cipher_{}", size), |b| {
            let encrypt = knapsack.encrypt(text.clone());
            b.iter(|| {
                let decrypt = knapsack.decrypt(black_box(encrypt.clone()));
                black_box(decrypt);
            })
        });
    }
    group.finish();
}
