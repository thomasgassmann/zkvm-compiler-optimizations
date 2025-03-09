use criterion::{criterion_group, criterion_main, Criterion};
use runner::types::Config;
use std::env;

mod benchmarks;

use crate::benchmarks::utils::add_benchmarks_for;
use crate::benchmarks::utils::read_config_json;

fn benches_setup(c: &mut Criterion) {
    env::set_current_dir(env::current_dir().unwrap().parent().unwrap()).unwrap();
    let config: Config = read_config_json();

    for program in config.programs.list.iter() {
        for measurement in config.measurements.iter() {
            for prover in config.zkvms.iter() {
                let mut group = c.benchmark_group(&format!("{}-{}-{}", program, prover, measurement));
                group.sample_size(10);

                for (profile, _) in config.profiles.iter() {
                    add_benchmarks_for(&program, &prover, &mut group, measurement, profile);
                }
                
                group.finish();
            }
        }
    }
}

criterion_group!(benches, benches_setup);
criterion_main!(benches);
