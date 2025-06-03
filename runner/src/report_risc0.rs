use risc0_zkvm::{
    compute_image_id, get_prover_server, ExecutorEnv, ExecutorImpl, ProverOpts, VerifierContext,
};

use crate::{
    input::set_risc0_input,
    types::{PerformanceReport, ProgramId, ProverId},
    utils::time_operation,
};

// adapted from https://github.com/succinctlabs/zkvm-perf
pub struct Risc0Evaluator;

impl Risc0Evaluator {
    pub fn eval(elf: &Vec<u8>, program: &ProgramId) -> PerformanceReport {
        let image_id = compute_image_id(elf.as_slice()).unwrap();

        let mut builder = ExecutorEnv::builder();
        set_risc0_input(&program, &mut builder, &None);
        let env = builder.build().unwrap();

        // Compute some statistics.
        let mut exec = ExecutorImpl::from_elf(env, &elf).unwrap();
        let session = exec.run().unwrap();
        let cycles = session.user_cycles;
        println!("cycles: {}", cycles);
        println!("cycles (paging): {}", session.paging_cycles);

        // Setup the prover.
        let mut builder = ExecutorEnv::builder();
        set_risc0_input(&program, &mut builder, &None);
        let env = builder.build().unwrap();

        // Generate the session.
        let mut exec = ExecutorImpl::from_elf(env, &elf).unwrap();
        let (session, execution_duration) = time_operation(|| exec.run().unwrap());

        // Generate the proof.
        let opts = ProverOpts::default();
        let prover = get_prover_server(&opts).unwrap();
        let ctx = VerifierContext::default();
        let (info, core_prove_duration) =
            time_operation(|| prover.prove_session(&ctx, &session).unwrap());
        let receipt = info.receipt;

        let composite_receipt = receipt.inner.composite().unwrap();
        let num_segments = composite_receipt.segments.len();

        // Get the core proof size by summing across all segments.
        let mut core_proof_size = 0;
        for segment in composite_receipt.segments.iter() {
            core_proof_size += segment.seal.len() * 4;
        }

        // Verify the core proof.
        let ((), core_verify_duration) = time_operation(|| receipt.verify(image_id).unwrap());

        // Now compress the proof with recursion.
        let (compressed_proof, compress_duration) =
            time_operation(|| prover.compress(&ProverOpts::succinct(), &receipt).unwrap());

        // Verify the recursive proof
        let ((), recursive_verify_duration) =
            time_operation(|| compressed_proof.verify(image_id).unwrap());

        let succinct_receipt = compressed_proof.inner.succinct().unwrap();

        // Get the recursive proof size.
        let recursive_proof_size = succinct_receipt.seal.len() * 4;
        let prove_duration = core_prove_duration + compress_duration;

        let core_khz = cycles as f64 / core_prove_duration.as_secs_f64() / 1_000.0;
        let overall_khz = cycles as f64 / prove_duration.as_secs_f64() / 1_000.0;

        // Create the performance report.
        PerformanceReport {
            program: program.to_string(),
            prover: ProverId::Risc0.to_string(),
            shards: num_segments,
            cycles: cycles as u64,
            speed: (cycles as f64) / prove_duration.as_secs_f64(),
            execution_duration: execution_duration.as_secs_f64(),
            prove_duration: prove_duration.as_secs_f64(),
            core_prove_duration: core_prove_duration.as_secs_f64(),
            core_verify_duration: core_verify_duration.as_secs_f64(),
            core_proof_size,
            core_khz,
            compress_prove_duration: compress_duration.as_secs_f64(),
            compress_verify_duration: recursive_verify_duration.as_secs_f64(),
            compress_proof_size: recursive_proof_size,
            overall_khz,
        }
    }
}
