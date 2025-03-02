use crate::{
    utils::{read_elf, time_operation},
    EvalArgs, PerformanceReport, ProgramId,
};

use sp1_core_executor::SP1Context;
use sp1_core_machine::io::SP1Stdin;
use sp1_prover::{components::CpuProverComponents, utils::get_cycles, SP1Prover};
use sp1_stark::SP1ProverOpts;

pub struct SP1Evaluator;

impl SP1Evaluator {
    pub fn eval(args: &EvalArgs) -> PerformanceReport {
        // Get stdin.
        let mut stdin = SP1Stdin::new();
        match args.program {
            ProgramId::Factorial => {
                stdin.write::<u32>(&10);
            }
            ProgramId::Keccak256 => {
                stdin.write(&vec![0u8; 64]);
            }
            _ => {}
        }

        let elf = read_elf(&args.program, &args.prover);

        let cycles = get_cycles(&elf, &stdin);
        println!("cycles: {}", cycles);

        let prover = SP1Prover::<CpuProverComponents>::new();

        // Setup the program.
        let (_, pk_d, program, vk) = prover.setup(&elf);

        // Execute the program.
        let context = SP1Context::default();
        let (_, execution_duration) =
            time_operation(|| prover.execute(&elf, &stdin, context.clone()).unwrap());

        // Setup the prover opionts.
        let opts = SP1ProverOpts::auto();

        // Generate the core proof (CPU).
        let (core_proof, prove_core_duration) = time_operation(|| {
            prover
                .prove_core(&pk_d, program, &stdin, opts, context)
                .unwrap()
        });

        let num_shards = core_proof.proof.0.len();

        // Verify the proof.
        let core_bytes = bincode::serialize(&core_proof).unwrap();
        let (_, verify_core_duration) = time_operation(|| {
            prover
                .verify(&core_proof.proof, &vk)
                .expect("Proof verification failed")
        });

        let (compress_proof, compress_duration) =
            time_operation(|| prover.compress(&vk, core_proof, vec![], opts).unwrap());

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
            program: args.program.to_string(),
            prover: args.prover.to_string(),
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
