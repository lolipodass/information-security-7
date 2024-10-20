extern crate criterion;
use caesars::benchmark_caesars;
use criterion::{ criterion_group, criterion_main };
use double_permutation::benchmark_double_permutation;
use enigma::enigma_benchmark;
use route::benchmark_route_permutation;
use trisemus::benchmark_trisemus;

mod caesars;
mod enigma;
mod trisemus;
mod route;
mod double_permutation;

criterion_group!(
    benches,
    benchmark_caesars,
    enigma_benchmark,
    benchmark_trisemus,
    benchmark_route_permutation,
    benchmark_double_permutation
);
criterion_main!(benches);
