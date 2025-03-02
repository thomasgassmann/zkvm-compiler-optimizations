use criterion::{measurement::WallTime, Criterion};
use runner::{types::{ProgramId, ProverId}, utils::read_elf};
use sp1_prover::components::CpuProverComponents;
use sp1_sdk::{SP1Context, SP1Prover};

use crate::benchmarks::risc0_utils::get_risc0_executor;
use crate::benchmarks::input::get_sp1_stdin;

pub fn add_benchmarks_for(program: ProgramId, prover: ProverId, c: &mut Criterion) {
    let mut group = c.benchmark_group(&format!("{}-{}", program, prover));

    match prover {
        ProverId::Risc0 => add_risc0_exec("execute", &mut group, &program),
        ProverId::SP1 => add_sp1_exec("execute", &mut group, &program),
    }

    // group.bench_function("core_prove", |b| {
    //     b.iter(|| 1 + 1);
    // });

    // group.bench_function("core_verify", |b| {
    //     b.iter(|| 1 + 1);
    // });

    // group.bench_function("compress", |b| {
    //     b.iter(|| 1 + 1);
    // });

    // group.bench_function("compress_verify", |b| {
    //     b.iter(|| 1 + 1);
    // });
}

fn add_sp1_exec(name: &str, group: &mut criterion::BenchmarkGroup<'_, WallTime>, program: &ProgramId) {
    let stdin = get_sp1_stdin(program);
    let elf = read_elf(program, &ProverId::SP1);

    let prover = SP1Prover::<CpuProverComponents>::new();
    let (_, _, _, _) = prover.setup(&elf);

    group.bench_function(name, |b| {
        b.iter(|| prover.execute(&elf, &stdin, SP1Context::default()).unwrap());
    });
}

fn add_risc0_exec(name: &str, group: &mut criterion::BenchmarkGroup<'_, WallTime>, program: &ProgramId) {
    let mut p = get_risc0_executor(program);
    
    group.bench_function(name, |b| {
        b.iter(|| p.run().unwrap());
    });
}
