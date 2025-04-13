use sp1_core_executor::SP1Context;
use sp1_prover::{components::CpuProverComponents, utils::get_cycles, SP1Prover};
use sp1_stark::SP1ProverOpts;

use crate::{
    input::get_sp1_stdin,
    types::{PerformanceReport, ProgramId, ProverId},
    utils::time_operation,
};

#[cfg(feature = "cuda")]
use sp1_cuda::SP1CudaProver;

// adapted from https://github.com/succinctlabs/zkvm-perf
pub struct SP1Evaluator;

impl SP1Evaluator {
    pub fn eval(elf: &Vec<u8>, program_id: &ProgramId) -> PerformanceReport {
        // Get stdin.
        let stdin = get_sp1_stdin(&program_id);

        // Get the elf.
        let cycles = get_cycles(&elf, &stdin);
        println!("cycles: {}", cycles);

        let prover = SP1Prover::<CpuProverComponents>::new();

        #[cfg(feature = "cuda")]
        let server = SP1CudaProver::new(None).expect("Failed to initialize CUDA prover");

        // Setup the program.
        #[cfg(not(feature = "cuda"))]
        let (_, pk_d, program, vk) = prover.setup(&elf);

        #[cfg(feature = "cuda")]
        let (pk, vk) = server.setup(&elf).unwrap();

        // Execute the program.
        let context = SP1Context::default();
        let ((_pv, _), execution_duration) =
            time_operation(|| prover.execute(&elf, &stdin, context.clone()).unwrap());

        // Setup the prover opionts.
        #[cfg(not(feature = "cuda"))]
        let opts = SP1ProverOpts::auto();

        // Generate the core proof (CPU).
        #[cfg(not(feature = "cuda"))]
        let (core_proof, prove_core_duration) = time_operation(|| {
            prover
                .prove_core(&pk_d, program, &stdin, opts, context)
                .unwrap()
        });

        // Generate the core proof (CUDA).
        #[cfg(feature = "cuda")]
        let (core_proof, prove_core_duration) =
            time_operation(|| server.prove_core(&stdin).unwrap());

        let num_shards = core_proof.proof.0.len();

        // Verify the proof.
        let core_bytes = bincode::serialize(&core_proof).unwrap();
        let (_, verify_core_duration) = time_operation(|| {
            prover
                .verify(&core_proof.proof, &vk)
                .expect("Proof verification failed")
        });

        #[cfg(not(feature = "cuda"))]
        let (compress_proof, compress_duration) =
            time_operation(|| prover.compress(&vk, core_proof, vec![], opts).unwrap());

        #[cfg(feature = "cuda")]
        let (compress_proof, compress_duration) =
            time_operation(|| server.compress(&vk, core_proof, vec![]).unwrap());

        let compress_bytes = bincode::serialize(&compress_proof).unwrap();
        println!("recursive proof size: {}", compress_bytes.len());

        let (_, verify_compress_duration) = time_operation(|| {
            prover
                .verify_compressed(&compress_proof, &vk)
                .expect("Proof verification failed")
        });

        let prove_duration = prove_core_duration + compress_duration;
        let core_khz = cycles as f64 / prove_core_duration.as_secs_f64() / 1_000.0;
        let overall_khz = cycles as f64 / prove_duration.as_secs_f64() / 1_000.0;

        // Create the performance report.
        let report = PerformanceReport {
            program: program_id.to_string(),
            prover: ProverId::SP1.to_string(),
            shards: num_shards,
            cycles: cycles as u64,
            speed: (cycles as f64) / prove_core_duration.as_secs_f64(),
            execution_duration: execution_duration.as_secs_f64(),
            prove_duration: prove_duration.as_secs_f64(),
            core_prove_duration: prove_core_duration.as_secs_f64(),
            core_verify_duration: verify_core_duration.as_secs_f64(),
            core_proof_size: core_bytes.len(),
            core_khz,
            compress_prove_duration: compress_duration.as_secs_f64(),
            compress_verify_duration: verify_compress_duration.as_secs_f64(),
            compress_proof_size: compress_bytes.len(),
            overall_khz,
        };

        report
    }
}
