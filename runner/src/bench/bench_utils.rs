use super::risc0_utils::{
    exec_risc0, exec_risc0_setup, get_risc0_stats, prove_core_risc0, prove_core_risc0_prepare,
};
use crate::bench::sp1_utils::{exec_sp1, get_sp1_stats, prove_core_sp1, prove_core_sp1_prepare};
use crate::bench::utils::write_elf_stats;
use criterion::measurement::WallTime;
use criterion::BenchmarkId;
use super::super::{
    types::{ProgramId, ProverId, MeasurementType},
    utils::read_elf,
};

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
    write_elf_stats(
        program,
        &ProverId::SP1,
        profile,
        &get_sp1_stats(&elf, program),
    );
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
    write_elf_stats(
        program,
        &ProverId::Risc0,
        profile,
        &get_risc0_stats(&elf, program),
    );

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
