use criterion::{measurement::WallTime, Criterion};
use runner::{
    types::{ProgramId, ProverId},
    utils::read_elf,
};

use crate::benchmarks::risc0_utils::exec_risc0;
use crate::benchmarks::sp1_utils::exec_sp1;

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

fn add_sp1_exec(
    name: &str,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
) {
    let elf = read_elf(program, &ProverId::SP1);

    group.bench_function(name, |b| {
        b.iter(|| exec_sp1(&elf, program));
    });
}

fn add_risc0_exec(
    name: &str,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
) {
    let elf = read_elf(program, &ProverId::Risc0);

    group.bench_function(name, |b| {
        b.iter(|| exec_risc0(&elf, program));
    });
}
