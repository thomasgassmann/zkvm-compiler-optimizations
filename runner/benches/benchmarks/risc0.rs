use criterion::{criterion_group, Criterion};
use runner::types::ProverId;

use crate::benchmarks::utils::run_all;

fn risc0_programs(c: &mut Criterion) {
    // benchmark all programs *in the current state* against risc0
    run_all(c, ProverId::Risc0);
}

criterion_group!(risc0, risc0_programs);
