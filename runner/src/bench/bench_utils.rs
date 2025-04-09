use std::env;

use super::risc0_utils::{
    exec_risc0, exec_risc0_setup, get_risc0_stats, prove_core_risc0, prove_core_risc0_prepare,
};
use super::{
    super::{
        types::{MeasurementType, ProgramId, ProverId},
        utils::read_elf,
    },
    sp1_utils::exec_sp1_prepare,
};
use crate::bench::sp1_utils::{exec_sp1, get_sp1_stats, prove_core_sp1, prove_core_sp1_prepare};
use crate::bench::utils::write_elf_stats;
use crate::input::get_sp1_stdin;
use crate::utils::is_gpu_proving;
use criterion::measurement::WallTime;
use sp1_sdk::ProverClient;

pub fn add_benchmarks_for(
    program: &ProgramId,
    prover: &ProverId,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    measurement: &MeasurementType,
    profile: &String,
    meta_only: bool,
) {
    match prover {
        ProverId::Risc0 => {
            add_risc0_exec_and_prove(group, program, measurement, profile, meta_only)
        }
        ProverId::SP1 => add_sp1_exec_and_prove(group, program, measurement, profile, meta_only),
    }
}

fn add_sp1_exec_and_prove(
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
    measurement: &MeasurementType,
    profile: &String,
    meta_only: bool,
) {
    let elf = read_elf(program, &ProverId::SP1, profile);
    write_elf_stats(
        program,
        &ProverId::SP1,
        profile,
        &get_sp1_stats(&elf, program),
    );
    if meta_only {
        return;
    }


    env::set_var("SP1_PROVER", if is_gpu_proving() { "cuda" } else { "cpu" });

    let prover = ProverClient::from_env();
    let (pk, _) = prover.setup(&elf);
    let stdin = get_sp1_stdin(program);
    prover.prove(&pk, &stdin).core().run().unwrap();
    return;

    let (pk, _, stdin) = prove_core_sp1_prepare(&elf, program);
    match measurement {
        MeasurementType::Exec => {
            group.bench_function(profile, |b| {
                b.iter_with_setup(
                    || exec_sp1_prepare(&elf, program),
                    |(stdin, prover)| exec_sp1(&stdin, &prover, &elf),
                );
            });
        }
        MeasurementType::Prove => {
            group.bench_function(profile, |b| {
                b.iter_with_setup(
                    || pk.clone(),
                    |cloned_pk| prove_core_sp1(&stdin, &cloned_pk),
                );
            });
        }
    }
}

fn add_risc0_exec_and_prove(
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
    measurement: &MeasurementType,
    profile: &String,
    meta_only: bool,
) {
    let elf = read_elf(program, &ProverId::Risc0, profile);
    write_elf_stats(
        program,
        &ProverId::Risc0,
        profile,
        &get_risc0_stats(&elf, program),
    );
    if meta_only {
        return;
    }

    match measurement {
        MeasurementType::Exec => {
            group.bench_function(profile, |b| {
                b.iter_with_setup(
                    || exec_risc0_setup(&elf, program),
                    |mut executor| exec_risc0(&mut executor),
                );
            });
        }
        MeasurementType::Prove => {
            let (prover, ctx, session) = prove_core_risc0_prepare(&elf, program);
            group.bench_function(profile, |b| {
                b.iter(|| prove_core_risc0(&prover, &ctx, &session));
            });
        }
    }
}
