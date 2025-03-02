use criterion::{criterion_group, Criterion};
use runner::types::{ProgramId, ProverId};
use crate::benchmarks::utils::add_benchmarks_for;


fn sp1_programs(c: &mut Criterion) {
    // benchmark all programs *in the current state* against sp1
    add_benchmarks_for(ProgramId::Factorial, ProverId::SP1, c);
    add_benchmarks_for(ProgramId::Keccak256, ProverId::SP1, c);
    add_benchmarks_for(ProgramId::LoopSum, ProverId::SP1, c);
    add_benchmarks_for(ProgramId::RustTests, ProverId::SP1, c);
    add_benchmarks_for(ProgramId::Sha256, ProverId::SP1, c);
}

criterion_group!(sp1, sp1_programs);
