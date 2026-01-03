use std::env;
use std::sync::Arc;

use crate::utils::is_gpu_proving;

use super::{
    super::{input::get_sp1_stdin, types::ProgramId},
    utils::get_elf_hash,
};
use once_cell::sync::Lazy;
use sp1_core_executor::{ExecutionRecord, Program};
use sp1_prover::components::CpuProverComponents;
use sp1_sdk::{
    EnvProver, ExecutionReport, Executor, ProverClient, SP1Context, SP1Prover, SP1ProvingKey,
    SP1PublicValues, SP1Stdin, SP1VerifyingKey,
};
use sp1_stark::{MachineRecord, SP1ProverOpts};

use super::utils::ElfStats;

static ENV_PROVER_CLIENT: Lazy<EnvProver> = Lazy::new(|| {
    if !env::var("SP1_PROVER").is_ok() {
        env::set_var("SP1_PROVER", if is_gpu_proving() { "cuda" } else { "cpu" });
    }
    let prover = ProverClient::from_env();
    prover
});

#[inline(always)]
pub fn exec_sp1_prepare(
    elf: &[u8],
    program: &ProgramId,
    input_override: &Option<String>,
) -> (SP1Stdin, SP1Prover<CpuProverComponents>) {
    let stdin = get_sp1_stdin(program, input_override);

    let prover = SP1Prover::<CpuProverComponents>::new();
    let (_, _, _, _) = prover.setup(&elf);
    (stdin, prover)
}

fn get_sp1_metrics_combined(
    elf: &[u8],
    program: &ProgramId,
    input_override: &Option<String>,
) -> (u64, u64, u64) {
    let stdin = get_sp1_stdin(program, input_override);
    let prover = SP1Prover::<CpuProverComponents>::new();
    let (_, _, program, _) = prover.setup(&elf);

    let opts = SP1ProverOpts::auto().core_opts;

    let mut executor = Executor::with_context(program.clone(), opts.clone(), SP1Context::default());
    executor.maximal_shapes = prover.core_shape_config.as_ref().map(|config| {
        config
            .maximal_core_shapes(opts.shard_size.ilog2() as usize)
            .into_iter()
            .collect()
    });
    executor.record_estimator = Some(Box::default());

    executor.write_vecs(&stdin.buffer);
    for (proof, vkey) in stdin.proofs.iter() {
        executor.write_proof(proof.clone(), vkey.clone());
    }

    let mut total_shards = 0;
    let mut total_instructions = 0;
    let mut total_cycles = 0;
    let mut deferred_acc = ExecutionRecord::new(Arc::new(program.clone()));

    loop {
        let (checkpoint_state, _, done) = executor.execute_state(false).expect("Execution failed");
        let num_cycles = executor.state.global_clk;
        if done {
            total_cycles = num_cycles;
        }

        let mut runtime = Executor::recover(program.clone(), checkpoint_state, opts.clone());
        runtime.maximal_shapes = prover.core_shape_config.as_ref().map(|config| {
            config
                .maximal_core_shapes(opts.shard_size.ilog2() as usize)
                .into_iter()
                .collect()
        });

        let (mut records, _) = runtime
            .execute_record(true)
            .expect("Trace execution failed");
        total_instructions += runtime.report.total_instruction_count();

        for record in records.iter_mut() {
            deferred_acc.append(&mut record.defer());
        }

        let should_combine = done
            && num_cycles < 1 << 21
            && deferred_acc.global_memory_initialize_events.len()
                < opts.split_opts.combine_memory_threshold
            && deferred_acc.global_memory_finalize_events.len()
                < opts.split_opts.combine_memory_threshold;

        let last_record = if should_combine {
            records.last_mut().map(|b| b.as_mut())
        } else {
            None
        };
        let extra_shards = deferred_acc.split(done, last_record, opts.split_opts);

        total_shards += records.len();
        total_shards += extra_shards.len();

        if done {
            break;
        }
    }

    (total_shards as u64, total_cycles, total_instructions)
}

pub fn get_sp1_stats(elf: &[u8], program: &ProgramId, input_override: &Option<String>) -> ElfStats {
    let (shards, cycles, instructions) = get_sp1_metrics_combined(elf, program, input_override);
    ElfStats {
        dynamic_instruction_count: Some(instructions),
        cycle_count: Some(cycles),
        paging_cycles: None,
        reserved_cycles: None,
        total_cycles: None,
        shards: Some(shards),
        size: elf.len(),
        hash: get_elf_hash(elf),
    }
}

#[inline(always)]
pub fn exec_sp1_bench(stdin: &SP1Stdin, elf: &[u8]) -> () {
    // taken from sp1_prover::SP1Prover::execute
    let context = SP1Context::default();

    let (opts, program) = (
        sp1_stark::SP1CoreOpts::default(),
        Program::from(elf).unwrap(),
    );
    let mut runtime = Executor::with_context(program, opts, context);

    runtime.write_vecs(&stdin.buffer);
    for (proof, vkey) in stdin.proofs.iter() {
        runtime.write_proof(proof.clone(), vkey.clone());
    }
    runtime.run_fast().unwrap();
}

#[inline(always)]
pub fn exec_sp1(
    stdin: &SP1Stdin,
    prover: &SP1Prover<CpuProverComponents>,
    elf: &[u8],
) -> (SP1PublicValues, ExecutionReport) {
    prover.execute(&elf, stdin, SP1Context::default()).unwrap()
}

pub fn prove_core_sp1_prepare(
    elf: &[u8],
    program: &ProgramId,
    input_override: &Option<String>,
) -> (SP1ProvingKey, SP1VerifyingKey, SP1Stdin) {
    let stdin = get_sp1_stdin(program, input_override);
    let (pk, vk) = ENV_PROVER_CLIENT.setup(elf);
    (pk, vk, stdin)
}

pub fn prove_core_sp1(stdin: &SP1Stdin, pk: &SP1ProvingKey) {
    ENV_PROVER_CLIENT.prove(pk, stdin).core().run().unwrap();
}

// #[allow(dead_code)]
// pub fn verify_core_sp1_prepare(
//     elf: &[u8],
//     program: &ProgramId,
// ) -> (
//     SP1Prover<CpuProverComponents>,
//     SP1ProofWithMetadata<SP1CoreProofData>,
//     SP1VerifyingKey,
//     SP1ProverOpts,
// ) {
//     let (stdin, prover, program, pk_d, opts, vk) = prove_core_sp1_prepare(elf);

//     let core_proof = prover
//         .prove_core(&pk_d, program, &stdin, opts, SP1Context::default())
//         .unwrap();
//     (prover, core_proof, vk, opts)
// }

// #[allow(dead_code)]
// pub fn verify_core_sp1(
//     prover: &SP1Prover<CpuProverComponents>,
//     core_proof: &SP1ProofWithMetadata<SP1CoreProofData>,
//     vk: &SP1VerifyingKey,
// ) {
//     prover
//         .verify(&core_proof.proof, vk)
//         .expect("Proof verification failed")
// }

// #[allow(dead_code)]
// pub fn compress_sp1_prepare(
//     elf: &[u8],
//     program: &ProgramId,
// ) -> (
//     SP1Prover<CpuProverComponents>,
//     SP1ProofWithMetadata<SP1CoreProofData>,
//     SP1VerifyingKey,
//     SP1ProverOpts,
// ) {
//     verify_core_sp1_prepare(elf, program)
// }

// #[allow(dead_code)]
// pub fn compress_sp1(
//     prover: &SP1Prover<CpuProverComponents>,
//     core_proof: SP1ProofWithMetadata<SP1CoreProofData>,
//     vk: &SP1VerifyingKey,
//     opts: SP1ProverOpts,
// ) -> SP1ReduceProof<BabyBearPoseidon2> {
//     prover.compress(vk, core_proof, vec![], opts).unwrap()
// }

// #[allow(dead_code)]
// pub fn compress_verify_sp1_prepare(
//     elf: &[u8],
//     program: &ProgramId,
// ) -> (
//     SP1Prover<CpuProverComponents>,
//     SP1ReduceProof<BabyBearPoseidon2>,
//     SP1VerifyingKey,
// ) {
//     let (prover, core_proof, vk, opts) = compress_sp1_prepare(elf, program);
//     let compress_proof = compress_sp1(&prover, core_proof, &vk, opts);
//     (prover, compress_proof, vk)
// }

// #[allow(dead_code)]
// pub fn compress_verify_sp1(
//     prover: &SP1Prover<CpuProverComponents>,
//     compress_proof: &SP1ReduceProof<BabyBearPoseidon2>,
//     vk: &SP1VerifyingKey,
// ) {
//     prover
//         .verify_compressed(&compress_proof, &vk)
//         .expect("Proof verification failed");
// }
