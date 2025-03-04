use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

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
use criterion::measurement::WallTime;
use criterion::BenchmarkId;
use runner::{
    types::{ProgramId, ProverId},
    utils::read_elf,
};
use serde_json::from_reader;

pub fn read_config_json() -> HashMap<String, String> {
    let file = File::open("flags.json").expect("could not read config file");
    let reader = BufReader::new(file);

    from_reader(reader).expect("Failed to parse JSON")
}

pub fn add_benchmarks_for(
    program: &ProgramId,
    prover: &ProverId,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    profile: &String,
) {
    match prover {
        ProverId::Risc0 => add_risc0_exec(
            BenchmarkId::new(format!("{}-execute", prover), profile),
            group,
            program,
            profile,
        ),
        ProverId::SP1 => add_sp1_exec(
            BenchmarkId::new(format!("{}-execute", prover), profile),
            group,
            program,
            profile,
        ),
    }

    match prover {
        ProverId::Risc0 => add_risc0_core_prove(
            BenchmarkId::new(format!("{}-core_prove", prover), profile),
            group,
            program,
            profile,
        ),
        ProverId::SP1 => add_sp1_core_prove(
            BenchmarkId::new(format!("{}-core_prove", prover), profile),
            group,
            program,
            profile,
        ),
    }

    match prover {
        ProverId::Risc0 => add_risc0_core_verify(
            BenchmarkId::new(format!("{}-core_verify", prover), profile),
            group,
            program,
            profile,
        ),
        ProverId::SP1 => add_sp1_core_verify(
            BenchmarkId::new(format!("{}-core_verify", prover), profile),
            group,
            program,
            profile,
        ),
    }

    match prover {
        ProverId::Risc0 => add_risc0_compress(
            BenchmarkId::new(format!("{}-compress", prover), profile),
            group,
            program,
            profile,
        ),
        ProverId::SP1 => add_sp1_compress(
            BenchmarkId::new(format!("{}-compress", prover), profile),
            group,
            program,
            profile,
        ),
    }

    match prover {
        ProverId::Risc0 => add_risc0_compress_verify(
            BenchmarkId::new(format!("{}-compress_verify", prover), profile),
            group,
            program,
            profile,
        ),
        ProverId::SP1 => add_sp1_compress_verify(
            BenchmarkId::new(format!("{}-compress_verify", prover), profile),
            group,
            program,
            profile,
        ),
    }
}

fn add_sp1_compress_verify(
    name: BenchmarkId,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
    profile: &String,
) {
    let elf: Vec<u8> = read_elf(program, &ProverId::SP1, profile);
    let (prover, compressed_proof, vk) = compress_verify_sp1_prepare(&elf, program);

    group.bench_function(name, |b| {
        b.iter(|| compress_verify_sp1(&prover, &compressed_proof, &vk));
    });
}

fn add_risc0_compress_verify(
    name: BenchmarkId,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
    profile: &String,
) {
    let elf: Vec<u8> = read_elf(program, &ProverId::Risc0, profile);
    let (compressed_proof, image_id) = compress_verify_risc0_prepare(&elf, program);

    group.bench_function(name, |b| {
        b.iter(|| compress_verify_risc0(&compressed_proof, image_id));
    });
}

fn add_risc0_compress(
    name: BenchmarkId,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
    profile: &String,
) {
    let elf: Vec<u8> = read_elf(program, &ProverId::Risc0, profile);
    let (receipt, prover) = compress_risc0_prepare(&elf, program);

    group.bench_function(name, |b| {
        b.iter(|| compress_risc0(&receipt, &prover));
    });
}

fn add_sp1_compress(
    name: BenchmarkId,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
    profile: &String,
) {
    let elf: Vec<u8> = read_elf(program, &ProverId::SP1, profile);
    let (prover, proof, vk, opts) = compress_sp1_prepare(&elf, program);

    group.bench_function(name, |b| {
        b.iter(|| compress_sp1(&prover, proof.clone(), &vk, opts));
    });
}

fn add_sp1_core_verify(
    name: BenchmarkId,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
    profile: &String,
) {
    let elf: Vec<u8> = read_elf(program, &ProverId::SP1, profile);
    let (prover, proof, vk, _) = verify_core_sp1_prepare(&elf, program);

    group.bench_function(name, |b| {
        b.iter(|| verify_core_sp1(&prover, &proof, &vk));
    });
}

fn add_risc0_core_verify(
    name: BenchmarkId,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
    profile: &String,
) {
    let elf: Vec<u8> = read_elf(program, &ProverId::Risc0, profile);
    let (receipt, image_id, _) = verify_core_risc0_prepare(&elf, program);

    group.bench_function(name, |b| {
        b.iter(|| verify_core_risc0(&receipt, image_id));
    });
}

fn add_risc0_core_prove(
    name: BenchmarkId,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
    profile: &String,
) {
    let elf = read_elf(program, &ProverId::Risc0, profile);
    let (prover, ctx, session) = prove_core_risc0_prepare(&elf, program);

    group.bench_function(name, |b| {
        b.iter(|| prove_core_risc0(&prover, &ctx, &session));
    });
}

fn add_sp1_core_prove(
    name: BenchmarkId,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
    profile: &String,
) {
    let elf = read_elf(program, &ProverId::SP1, profile);
    let (stdin, prover, program, pk_d, opts, _) = prove_core_sp1_prepare(&elf, program);

    group.bench_function(name, |b| {
        b.iter(|| prove_core_sp1(&stdin, &prover, program.clone(), &pk_d, opts));
    });
}

fn add_sp1_exec(
    name: BenchmarkId,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
    profile: &String,
) {
    let elf = read_elf(program, &ProverId::SP1, profile);
    let (stdin, prover) = exec_sp1_prepare(&elf, program);

    group.bench_function(name, |b| {
        b.iter(|| exec_sp1(&stdin, &prover, &elf));
    });
}

fn add_risc0_exec(
    name: BenchmarkId,
    group: &mut criterion::BenchmarkGroup<'_, WallTime>,
    program: &ProgramId,
    profile: &String,
) {
    let elf = read_elf(program, &ProverId::Risc0, profile);

    group.bench_function(name, |b| {
        b.iter_with_setup(|| exec_risc0_setup(&elf, program), exec_risc0);
    });
}
