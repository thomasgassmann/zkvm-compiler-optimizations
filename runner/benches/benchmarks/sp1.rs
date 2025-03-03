use criterion::{criterion_group, Criterion};
use runner::types::ProverId;

use crate::benchmarks::utils::run_all;

fn sp1_programs(c: &mut Criterion) {
    // benchmark all programs *in the current state* against sp1
    run_all(c, ProverId::SP1);
}

criterion_group!(sp1, sp1_programs);
