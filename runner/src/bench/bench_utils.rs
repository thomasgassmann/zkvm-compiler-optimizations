use super::risc0_utils::{
    exec_risc0, exec_risc0_setup, get_risc0_stats, prove_core_risc0, prove_core_risc0_prepare,
};
use super::utils::is_same_as_baseline;
use super::{
    super::{
        types::{MeasurementType, ProgramId, ProverId},
        utils::read_elf,
    },
    sp1_utils::exec_sp1_prepare,
};
use crate::bench::sp1_utils::{exec_sp1, get_sp1_stats, prove_core_sp1, prove_core_sp1_prepare};
use crate::bench::utils::write_elf_stats;
use criterion::measurement::WallTime;

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
    if meta_only || is_same_as_baseline(program, &ProverId::SP1, profile) {
        return;
    }

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
    if meta_only || is_same_as_baseline(program, &ProverId::Risc0, profile) {
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
