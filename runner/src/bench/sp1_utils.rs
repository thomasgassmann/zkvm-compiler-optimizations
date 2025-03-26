use super::super::{input::get_sp1_stdin, types::ProgramId};
use sp1_core_executor::{IoWriter, Program, SP1ReduceProof};
use sp1_prover::{
    components::CpuProverComponents, SP1CoreProofData, SP1ProofWithMetadata,
};
use sp1_sdk::{Executor, SP1Context, SP1Prover, SP1Stdin, SP1VerifyingKey};
use sp1_stark::{baby_bear_poseidon2::BabyBearPoseidon2, SP1CoreOpts, SP1ProverOpts, StarkProvingKey};

use super::utils::ElfStats;

#[allow(dead_code)]
pub fn exec_sp1_prepare(
    elf: &[u8],
    program: &ProgramId,
) -> (SP1Stdin, SP1Prover<CpuProverComponents>) {
    let stdin = get_sp1_stdin(program);

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

pub fn get_sp1_stats(elf: &[u8], program: &ProgramId) -> ElfStats {
    let (stdin, _) = exec_sp1_prepare(elf, program);
    ElfStats {
        cycle_count: get_cycles(&elf, &stdin),
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

pub fn exec_sp1(stdin: &SP1Stdin, prover: &SP1Prover<CpuProverComponents>, elf: &[u8]) {
    let mut s = SP1StdoutSink;
    let context = SP1Context::builder()
        .stdout(&mut s)
        .build();
    prover.execute(&elf, stdin, context).unwrap();
}

pub fn prove_core_sp1_prepare(
    elf: &[u8],
    program: &ProgramId,
) -> (
    SP1Stdin,
    SP1Prover<CpuProverComponents>,
    Program,
    StarkProvingKey<BabyBearPoseidon2>,
    SP1ProverOpts,
    SP1VerifyingKey,
) {
    let stdin = get_sp1_stdin(program);
    let prover = SP1Prover::<CpuProverComponents>::new();
    let (_, pk_d, program, vk) = prover.setup(&elf);
    let opts = SP1ProverOpts::auto();
    (stdin, prover, program, pk_d, opts, vk)
}

pub fn prove_core_sp1(
    stdin: &SP1Stdin,
    prover: &SP1Prover<CpuProverComponents>,
    program: Program,
    proving_key: &StarkProvingKey<BabyBearPoseidon2>,
    opts: SP1ProverOpts,
) {
    let mut s = SP1StdoutSink;
    let context = SP1Context::builder()
        .stdout(&mut s)
        .build();
    prover
        .prove_core(proving_key, program, stdin, opts, context)
        .unwrap();
}

#[allow(dead_code)]
pub fn verify_core_sp1_prepare(
    elf: &[u8],
    program: &ProgramId,
) -> (
    SP1Prover<CpuProverComponents>,
    SP1ProofWithMetadata<SP1CoreProofData>,
    SP1VerifyingKey,
    SP1ProverOpts,
) {
    let (stdin, prover, program, pk_d, opts, vk) = prove_core_sp1_prepare(elf, program);

    let core_proof = prover
        .prove_core(&pk_d, program, &stdin, opts, SP1Context::default())
        .unwrap();
    (prover, core_proof, vk, opts)
}

#[allow(dead_code)]
pub fn verify_core_sp1(
    prover: &SP1Prover<CpuProverComponents>,
    core_proof: &SP1ProofWithMetadata<SP1CoreProofData>,
    vk: &SP1VerifyingKey,
) {
    prover
        .verify(&core_proof.proof, vk)
        .expect("Proof verification failed")
}

#[allow(dead_code)]
pub fn compress_sp1_prepare(
    elf: &[u8],
    program: &ProgramId,
) -> (
    SP1Prover<CpuProverComponents>,
    SP1ProofWithMetadata<SP1CoreProofData>,
    SP1VerifyingKey,
    SP1ProverOpts,
) {
    verify_core_sp1_prepare(elf, program)
}

#[allow(dead_code)]
pub fn compress_sp1(
    prover: &SP1Prover<CpuProverComponents>,
    core_proof: SP1ProofWithMetadata<SP1CoreProofData>,
    vk: &SP1VerifyingKey,
    opts: SP1ProverOpts,
) -> SP1ReduceProof<BabyBearPoseidon2> {
    prover.compress(vk, core_proof, vec![], opts).unwrap()
}

#[allow(dead_code)]
pub fn compress_verify_sp1_prepare(
    elf: &[u8],
    program: &ProgramId,
) -> (
    SP1Prover<CpuProverComponents>,
    SP1ReduceProof<BabyBearPoseidon2>,
    SP1VerifyingKey,
) {
    let (prover, core_proof, vk, opts) = compress_sp1_prepare(elf, program);
    let compress_proof = compress_sp1(&prover, core_proof, &vk, opts);
    (prover, compress_proof, vk)
}

#[allow(dead_code)]
pub fn compress_verify_sp1(
    prover: &SP1Prover<CpuProverComponents>,
    compress_proof: &SP1ReduceProof<BabyBearPoseidon2>,
    vk: &SP1VerifyingKey,
) {
    prover
        .verify_compressed(&compress_proof, &vk)
        .expect("Proof verification failed");
}
