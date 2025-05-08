use std::{
    env,
    sync::{Arc, Mutex},
};

use crate::utils::is_gpu_proving;

use super::{
    super::{input::get_sp1_stdin, types::ProgramId},
    utils::get_elf_hash,
};
use once_cell::sync::Lazy;
use sp1_core_executor::{IoWriter, Program};
use sp1_prover::components::CpuProverComponents;
use sp1_sdk::{
    EnvProver, ExecutionReport, Executor, ProverClient, SP1Context, SP1Prover, SP1ProvingKey,
    SP1PublicValues, SP1Stdin, SP1VerifyingKey,
};
use sp1_stark::SP1CoreOpts;

use super::utils::ElfStats;

static ENV_PROVER_CLIENT: Lazy<Arc<Mutex<EnvProver>>> = Lazy::new(|| {
    env::set_var("SP1_PROVER", if is_gpu_proving() { "cuda" } else { "cpu" });
    let prover = ProverClient::from_env();
    Arc::new(Mutex::new(prover))
});

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

fn get_cycles(elf: &[u8], stdin: &SP1Stdin) -> u64 {
    let mut sink = SP1StdoutSink;
    let writer: Option<&'_ mut dyn IoWriter> = Some(&mut sink);
    let program = Program::from(elf).unwrap();
    let mut runtime = Executor::new(program, SP1CoreOpts::default());
    runtime.write_vecs(&stdin.buffer);
    runtime.io_options.stdout = writer;
    runtime.run_fast().unwrap();
    runtime.state.global_clk
}

pub fn get_sp1_stats(elf: &[u8], program: &ProgramId, input_override: &Option<String>) -> ElfStats {
    let (stdin, _) = exec_sp1_prepare(elf, program, input_override);
    ElfStats {
        cycle_count: get_cycles(&elf, &stdin),
        paging_cycles: None,
        size: elf.len(),
        hash: get_elf_hash(elf),
    }
}

struct SP1StdoutSink;

impl std::io::Write for SP1StdoutSink {
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        Ok(_buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub fn exec_sp1(
    stdin: &SP1Stdin,
    prover: &SP1Prover<CpuProverComponents>,
    elf: &[u8],
) -> (SP1PublicValues, ExecutionReport) {
    let mut s = SP1StdoutSink;
    let context = SP1Context::builder().stdout(&mut s).build();
    prover.execute(&elf, stdin, context).unwrap()
}

pub fn prove_core_sp1_prepare(
    elf: &[u8],
    program: &ProgramId,
    input_override: &Option<String>,
) -> (SP1ProvingKey, SP1VerifyingKey, SP1Stdin) {
    let stdin = get_sp1_stdin(program, input_override);
    let (pk, vk) = ENV_PROVER_CLIENT.lock().unwrap().setup(elf);
    (pk, vk, stdin)
}

pub fn prove_core_sp1(stdin: &SP1Stdin, pk: &SP1ProvingKey) {
    ENV_PROVER_CLIENT
        .lock()
        .unwrap()
        .prove(pk, stdin)
        .core()
        .run()
        .unwrap();
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
