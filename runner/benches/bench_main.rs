use criterion::{criterion_group, criterion_main, Criterion};
use runner::types::ProgramId;
use runner::types::ProverId;
use std::env;

mod benchmarks;

use crate::benchmarks::utils::add_benchmarks_for;
use crate::benchmarks::utils::read_config_json;

fn benches_setup(c: &mut Criterion) {
    env::set_current_dir(env::current_dir().unwrap().parent().unwrap()).unwrap();
    let config = read_config_json();

    let programs = vec![
        ProgramId::Factorial,
        ProgramId::Keccak256,
        ProgramId::LoopSum,
        ProgramId::RustTests,
        ProgramId::Sha256,
    ];

    for program in programs.iter() {
        let mut group = c.benchmark_group(&format!("{}", program));
        group.sample_size(10);
        for (profile, _) in config.iter() {
            add_benchmarks_for(&program, &ProverId::SP1, &mut group, profile);
            add_benchmarks_for(&program, &ProverId::Risc0, &mut group, profile);
        }
        group.finish();
    }
}

criterion_group!(benches, benches_setup);
criterion_main!(benches);
