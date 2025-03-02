use criterion::{criterion_group, Criterion};
use runner::types::{ProgramId, ProverId};
use crate::benchmarks::utils::add_benchmarks_for;

fn risc0_programs(c: &mut Criterion) {
    // benchmark all programs *in the current state* against risc0
    add_benchmarks_for(ProgramId::Factorial, ProverId::Risc0, c);
    add_benchmarks_for(ProgramId::Keccak256, ProverId::Risc0, c);
    add_benchmarks_for(ProgramId::LoopSum, ProverId::Risc0, c);
    add_benchmarks_for(ProgramId::RustTests, ProverId::Risc0, c);
    add_benchmarks_for(ProgramId::Sha256, ProverId::Risc0, c);
}

criterion_group!(risc0, risc0_programs);
