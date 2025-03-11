use std::fs::File;
use std::io::BufReader;

use crate::bench::risc0_utils::{exec_risc0, prove_core_risc0, prove_core_risc0_prepare};
use crate::bench::sp1_utils::{exec_sp1, prove_core_sp1, prove_core_sp1_prepare};
use criterion::measurement::WallTime;
use criterion::BenchmarkId;
use runner::types::{Config, MeasurementType};
use runner::{
    types::{ProgramId, ProverId},
    utils::read_elf,
};
use serde_json::from_reader;

use super::risc0_utils::exec_risc0_setup;

pub fn read_config_json() -> Config {
    let file = File::open("config.json").expect("could not read config file");
    let reader = BufReader::new(file);

    from_reader(reader).expect("Failed to parse JSON")
}

pub fn add_benchmarks_for(
    program: &ProgramId,
    prover: &ProverId,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    measurement: &MeasurementType,
    profile: &String,
) {
    match prover {
        ProverId::Risc0 => add_risc0_exec_and_prove(
            BenchmarkId::new(format!("{}-{}", prover, MeasurementType::Exec), profile),
            BenchmarkId::new(format!("{}-{}", prover, MeasurementType::Prove), profile),
            group,
            program,
            measurement,
            profile,
        ),
        ProverId::SP1 => add_sp1_exec_and_prove(
            BenchmarkId::new(format!("{}-{}", prover, MeasurementType::Exec), profile),
            BenchmarkId::new(format!("{}-{}", prover, MeasurementType::Prove), profile),
            group,
            program,
            measurement,
            profile,
        ),
    }
}

fn add_sp1_exec_and_prove(
    execute_name: BenchmarkId,
    prove_name: BenchmarkId,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
    measurement: &MeasurementType,
    profile: &String,
) {
    let elf = read_elf(program, &ProverId::SP1, profile);
    let (stdin, prover, program, pk_d, opts, _) = prove_core_sp1_prepare(&elf, program);

    match measurement {
        MeasurementType::Exec => {
            group.bench_function(execute_name, |b| {
                b.iter(|| exec_sp1(&stdin, &prover, &elf));
            });
        }
        MeasurementType::Prove => {
            group.bench_function(prove_name, |b| {
                b.iter_with_setup(
                    || program.clone(),
                    |cloned_program| prove_core_sp1(&stdin, &prover, cloned_program, &pk_d, opts),
                );
            });
        }
    }
}

fn add_risc0_exec_and_prove(
    execute_name: BenchmarkId,
    prove_name: BenchmarkId,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
    measurement: &MeasurementType,
    profile: &String,
) {
    let elf = read_elf(program, &ProverId::Risc0, profile);

    match measurement {
        MeasurementType::Exec => {
            group.bench_function(execute_name, |b| {
                b.iter_with_setup(
                    || exec_risc0_setup(&elf, program),
                    |mut executor| exec_risc0(&mut executor),
                );
            });
        }
        MeasurementType::Prove => {
            let (prover, ctx, session) = prove_core_risc0_prepare(&elf, program);
            group.bench_function(prove_name, |b| {
                b.iter(|| prove_core_risc0(&prover, &ctx, &session));
            });
        }
    }
}
