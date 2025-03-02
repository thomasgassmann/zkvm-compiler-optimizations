mod risc0;
mod sp1;
mod types;
mod utils;

use clap::{command, Parser};
use serde::Serialize;
use types::*;
use std::{fs::File, io::Write};

#[derive(Parser, Clone)]
#[command(about = "Evaluate the performance of a zkVM on a program.")]
pub struct EvalArgs {
    #[arg(long)]
    program: ProgramId,
    #[arg(long)]
    prover: ProverId,
    #[arg(long)]
    filename: String,
}

/// The performance report of a zkVM on a program.
#[derive(Debug, Serialize, Default)]
pub struct PerformanceReport {
    /// The program that is being evaluated.
    pub program: String,
    /// The prover that is being evaluated.
    pub prover: String,
    /// The number of shards.
    pub shards: usize,
    /// The reported number of cycles.
    ///
    /// Note that this number may vary based on the zkVM.
    pub cycles: u64,
    /// The reported speed in cycles per second.
    pub speed: f64,
    /// The reported duration of the execution in seconds.
    pub execution_duration: f64,
    /// The reported duration of the prover in seconds.
    pub prove_duration: f64,

    /// The reported duration of the core proving time in seconds.
    pub core_prove_duration: f64,
    /// The reported duration of the verifier in seconds.
    pub core_verify_duration: f64,
    /// The size of the core proof.
    pub core_proof_size: usize,
    /// The speed of the core proving time in KHz.
    pub core_khz: f64,

    /// The reported duration of the recursive proving time in seconds.
    pub compress_prove_duration: f64,
    /// The reported duration of the verifier in seconds.
    pub compress_verify_duration: f64,
    /// The size of the recursive proof in bytes.
    pub compress_proof_size: usize,

    /// The overall speed in KHz.
    pub overall_khz: f64,
}

fn main() {
    // Setup the logger.
    sp1_core_machine::utils::setup_logger();

    let args = EvalArgs::parse();

    // Select the correct implementation based on the prover.
    let report = match args.prover {
        ProverId::Risc0 => risc0::Risc0Evaluator::eval(&args),
        ProverId::SP1 => sp1::SP1Evaluator::eval(&args),
    };

    let json_string = serde_json::to_string_pretty(&report).unwrap();
    let mut file = File::create(args.filename).unwrap();
    file.write_all(json_string.as_bytes()).unwrap();
    println!("{0}", json_string);
}
