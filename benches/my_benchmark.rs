extern crate criterion;
use criterion::{ criterion_group, criterion_main };

use caesars::benchmark_caesars;
use des::{ des_decrypt_benchmark, des_encrypt_benchmark };
use double_permutation::benchmark_double_permutation;
use enigma::enigma_benchmark;
use knapsack::knapsack_benchmark;
use rc4::rc4_benchmark;
use route::benchmark_route_permutation;
use trisemus::benchmark_trisemus;

mod caesars;
mod enigma;
mod trisemus;
mod route;
mod double_permutation;
mod des;
mod rc4;
mod knapsack;

criterion_group!(
    benches,
    benchmark_caesars,
    enigma_benchmark,
    benchmark_trisemus,
    benchmark_route_permutation,
    benchmark_double_permutation,
    des_decrypt_benchmark,
    des_encrypt_benchmark,
    rc4_benchmark,
    knapsack_benchmark
);
criterion_main!(benches);
