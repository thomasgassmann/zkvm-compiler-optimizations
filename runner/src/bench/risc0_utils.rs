use std::rc::Rc;

use super::{
    super::{input::set_risc0_input, types::ProgramId},
    utils::get_elf_hash,
};
use risc0_zkvm::{
    compute_image_id, get_prover_server, sha::Digest, ExecutorEnv, ExecutorImpl, ProveInfo,
    ProverOpts, ProverServer, Receipt, Session, VerifierContext,
};

use super::utils::ElfStats;

pub fn exec_risc0_setup<'a>(
    elf: &'a [u8],
    program: &'a ProgramId,
    input_override: &Option<String>,
) -> ExecutorImpl<'a> {
    let mut builder = ExecutorEnv::builder();
    builder.stdout(std::io::sink());
    set_risc0_input(program, &mut builder, input_override);
    let env = builder.build();
    ExecutorImpl::from_elf(env.unwrap(), elf).unwrap()
}

pub fn get_risc0_stats<'a>(
    elf: &'a [u8],
    program: &'a ProgramId,
    input_override: &Option<String>,
) -> ElfStats {
    let mut exec = exec_risc0_setup(elf, program, input_override);
    let session = exec.run().unwrap();
    ElfStats {
        cycle_count: session.user_cycles,
        paging_cycles: Some(session.paging_cycles),
        size: elf.len(),
        hash: get_elf_hash(elf),
    }
}

pub fn exec_risc0(p: &mut ExecutorImpl<'_>) {
    p.run().unwrap();
}

pub fn prove_core_risc0_prepare<'a>(
    elf: &'a [u8],
    program: &'a ProgramId,
    input_override: &Option<String>,
) -> (Rc<dyn ProverServer>, VerifierContext, Session) {
    let mut exec = exec_risc0_setup(elf, program, input_override);
    let session = exec.run().unwrap();

    let opts = ProverOpts::default();
    let prover = get_prover_server(&opts).unwrap();
    let ctx = VerifierContext::default();
    (prover, ctx, session)
}

pub fn prove_core_risc0(
    prover: &Rc<dyn ProverServer>,
    ctx: &VerifierContext,
    session: &Session,
) -> ProveInfo {
    prover.prove_session(ctx, session).unwrap()
}

#[allow(dead_code)]
pub fn verify_core_risc0_prepare(
    elf: &[u8],
    program: &ProgramId,
    input_override: &Option<String>,
) -> (Receipt, Digest, Rc<dyn ProverServer>) {
    let image_id = compute_image_id(elf).unwrap();

    let (prover, ctx, session) = prove_core_risc0_prepare(elf, program, input_override);
    let info = prove_core_risc0(&prover, &ctx, &session);

    let receipt = info.receipt;
    (receipt, image_id, prover)
}

#[allow(dead_code)]
pub fn verify_core_risc0(receipt: &Receipt, image_id: Digest) {
    receipt.verify(image_id).unwrap();
}

#[allow(dead_code)]
pub fn compress_risc0_prepare(
    elf: &[u8],
    program: &ProgramId,
    input_override: &Option<String>,
) -> (Receipt, Rc<dyn ProverServer>) {
    let (receipt, _, prover) = verify_core_risc0_prepare(elf, program, input_override);
    (receipt, prover)
}

#[allow(dead_code)]
pub fn compress_risc0(receipt: &Receipt, prover: &Rc<dyn ProverServer>) -> Receipt {
    prover.compress(&ProverOpts::succinct(), &receipt).unwrap()
}

#[allow(dead_code)]
pub fn compress_verify_risc0_prepare(
    elf: &[u8],
    program: &ProgramId,
    input_override: &Option<String>,
) -> (Receipt, Digest) {
    let image_id = compute_image_id(elf).unwrap();
    let (receipt, prover) = compress_risc0_prepare(elf, program, input_override);
    let compressed_receipt = compress_risc0(&receipt, &prover);
    (compressed_receipt, image_id)
}

#[allow(dead_code)]
pub fn compress_verify_risc0(compressed_proof: &Receipt, image_id: Digest) {
    compressed_proof.verify(image_id).unwrap();
}
