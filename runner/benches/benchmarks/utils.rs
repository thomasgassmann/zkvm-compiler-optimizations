use criterion::{measurement::WallTime, Criterion};
use runner::{
    types::{ProgramId, ProverId},
    utils::read_elf,
};

use crate::benchmarks::risc0_utils::{
    compress_risc0, compress_risc0_prepare, compress_verify_risc0, compress_verify_risc0_prepare,
    exec_risc0, exec_risc0_setup, prove_core_risc0, prove_core_risc0_prepare, verify_core_risc0,
    verify_core_risc0_prepare,
};
use crate::benchmarks::sp1_utils::{
    compress_sp1, compress_sp1_prepare, compress_verify_sp1, compress_verify_sp1_prepare, exec_sp1,
    exec_sp1_prepare, prove_core_sp1, prove_core_sp1_prepare, verify_core_sp1,
    verify_core_sp1_prepare,
};

pub fn add_benchmarks_for(program: ProgramId, prover: ProverId, c: &mut Criterion) {
    let mut group = c.benchmark_group(&format!("{}-{}", program, prover));

    match prover {
        ProverId::Risc0 => add_risc0_exec("execute", &mut group, &program),
        ProverId::SP1 => add_sp1_exec("execute", &mut group, &program),
    }

    match prover {
        ProverId::Risc0 => add_risc0_core_prove("core_prove", &mut group, &program),
        ProverId::SP1 => add_sp1_core_prove("core_prove", &mut group, &program),
    }

    match prover {
        ProverId::Risc0 => add_risc0_core_verify("core_verify", &mut group, &program),
        ProverId::SP1 => add_sp1_core_verify("core_verify", &mut group, &program),
    }

    match prover {
        ProverId::Risc0 => add_risc0_compress("compress", &mut group, &program),
        ProverId::SP1 => add_sp1_compress("compress", &mut group, &program),
    }

    match prover {
        ProverId::Risc0 => add_risc0_compress_verify("compress_verify", &mut group, &program),
        ProverId::SP1 => add_sp1_compress_verify("compress_verify", &mut group, &program),
    }
}

fn add_sp1_compress_verify(
    name: &str,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
) {
    let elf: Vec<u8> = read_elf(program, &ProverId::SP1);

    group.bench_function(name, |b| {
        b.iter_with_setup(
            || compress_verify_sp1_prepare(&elf, program),
            |(prover, compressed_proof, vk)| compress_verify_sp1(prover, compressed_proof, vk),
        );
    });
}

fn add_risc0_compress_verify(
    name: &str,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
) {
    let elf: Vec<u8> = read_elf(program, &ProverId::Risc0);

    group.bench_function(name, |b| {
        b.iter_with_setup(
            || compress_verify_risc0_prepare(&elf, program),
            |(compressed_proof, image_id)| compress_verify_risc0(compressed_proof, image_id),
        );
    });
}

fn add_risc0_compress(
    name: &str,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
) {
    let elf: Vec<u8> = read_elf(program, &ProverId::Risc0);

    group.bench_function(name, |b| {
        b.iter_with_setup(
            || compress_risc0_prepare(&elf, program),
            |(receipt, prover)| compress_risc0(receipt, prover),
        );
    });
}

fn add_sp1_compress(
    name: &str,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
) {
    let elf: Vec<u8> = read_elf(program, &ProverId::SP1);

    group.bench_function(name, |b| {
        b.iter_with_setup(
            || compress_sp1_prepare(&elf, program),
            |(prover, proof, vk, opts)| compress_sp1(&prover, proof, &vk, opts),
        );
    });
}

fn add_sp1_core_verify(
    name: &str,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
) {
    let elf: Vec<u8> = read_elf(program, &ProverId::SP1);

    group.bench_function(name, |b| {
        b.iter_with_setup(
            || verify_core_sp1_prepare(&elf, program),
            |(prover, proof, vk, _)| verify_core_sp1(prover, proof, vk),
        );
    });
}

fn add_risc0_core_verify(
    name: &str,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
) {
    let elf: Vec<u8> = read_elf(program, &ProverId::Risc0);

    group.bench_function(name, |b| {
        b.iter_with_setup(
            || verify_core_risc0_prepare(&elf, program),
            |(receipt, image_id, _)| verify_core_risc0(receipt, image_id),
        );
    });
}

fn add_risc0_core_prove(
    name: &str,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
) {
    let elf = read_elf(program, &ProverId::Risc0);

    group.bench_function(name, |b| {
        b.iter_with_setup(
            || prove_core_risc0_prepare(&elf, program),
            |(prover, ctx, session)| prove_core_risc0(&prover, ctx, session),
        );
    });
}

fn add_sp1_core_prove(
    name: &str,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
) {
    let elf = read_elf(program, &ProverId::SP1);

    group.bench_function(name, |b| {
        b.iter_with_setup(
            || prove_core_sp1_prepare(&elf, program),
            |(stdin, prover, program, pk_d, opts, _)| {
                prove_core_sp1(stdin, prover, program, pk_d, opts)
            },
        );
    });
}

fn add_sp1_exec(
    name: &str,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
) {
    let elf = read_elf(program, &ProverId::SP1);

    group.bench_function(name, |b| {
        b.iter_with_setup(
            || exec_sp1_prepare(&elf, program),
            |(stdin, prover)| exec_sp1(stdin, prover, &elf),
        );
    });
}

fn add_risc0_exec(
    name: &str,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
) {
    let elf = read_elf(program, &ProverId::Risc0);

    group.bench_function(name, |b| {
        b.iter_with_setup(|| exec_risc0_setup(&elf, program), exec_risc0);
    });
}
