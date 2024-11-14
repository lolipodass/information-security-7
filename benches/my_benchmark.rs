extern crate criterion;
use criterion::{ criterion_group, criterion_main };

use caesars::benchmark_caesars;
use des::{ des_decrypt_benchmark, des_encrypt_benchmark };
use double_permutation::benchmark_double_permutation;
use el_gamal::el_gamal_benchmark;
use enigma::enigma_benchmark;
use knapsack::knapsack_benchmark;
use md5::md5_benchmark;
use rc4::rc4_benchmark;
use route::benchmark_route_permutation;
use rsa::rsa_benchmark;
use trisemus::benchmark_trisemus;

mod caesars;
mod enigma;
mod trisemus;
mod route;
mod double_permutation;
mod des;
mod rc4;
mod knapsack;
mod rsa;
mod el_gamal;
mod md5;

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
    knapsack_benchmark,
    rsa_benchmark,
    el_gamal_benchmark,
    md5_benchmark
);
criterion_main!(benches);
