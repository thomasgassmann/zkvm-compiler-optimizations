use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use serde::{Deserialize, Serialize};

/// An identifier used to select the program to evaluate.
#[derive(clap::ValueEnum, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[clap(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum ProgramId {
    LoopSum,
    Factorial,
    Sha256,
    Keccak256,
    ZkvmMnist,
    Tailcall,
    Bigmem,
    Fibonacci,
    Sha2Bench,
    Sha2Chain,
    Sha3Bench,
    Sha3Chain,
    CSample,
    RegexMatch,
    Rsp,
}

/// An identifier used to select the prover to evaluate.
#[derive(clap::ValueEnum, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ProverId {
    #[serde(rename = "risc0")]
    Risc0,
    #[serde(rename = "sp1")]
    SP1,
}

#[derive(clap::ValueEnum, Clone, Debug, Serialize, Deserialize)]
pub enum MeasurementType {
    #[serde(rename = "prove")]
    Prove,
    #[serde(rename = "exec")]
    Exec,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramConfig {
    pub list: Vec<ProgramId>,
    pub specific: Vec<ProgramId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub rustflags: String,
    pub passes: Vec<String>,
    pub prepopulate_passes: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub profiles: HashMap<String, Profile>,
    pub zkvms: Vec<ProverId>,
    pub programs: ProgramConfig,
    pub measurements: Vec<MeasurementType>,
}

impl Display for ProgramId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramId::LoopSum => write!(f, "loop-sum"),
            ProgramId::Factorial => write!(f, "factorial"),
            ProgramId::Sha256 => write!(f, "sha256"),
            ProgramId::Keccak256 => write!(f, "keccak256"),
            ProgramId::ZkvmMnist => write!(f, "zkvm-mnist"),
            ProgramId::Tailcall => write!(f, "tailcall"),
            ProgramId::Bigmem => write!(f, "bigmem"),
            ProgramId::Fibonacci => write!(f, "fibonacci"),
            ProgramId::Sha2Bench => write!(f, "sha2-bench"),
            ProgramId::Sha2Chain => write!(f, "sha2-chain"),
            ProgramId::Sha3Bench => write!(f, "sha3-bench"),
            ProgramId::Sha3Chain => write!(f, "sha3-chain"),
            ProgramId::CSample => write!(f, "c-sample"),
            ProgramId::RegexMatch => write!(f, "regex-match"),
            ProgramId::Rsp => write!(f, "rsp"),
        }
    }
}

impl Display for ProverId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProverId::Risc0 => write!(f, "risc0"),
            ProverId::SP1 => write!(f, "sp1"),
        }
    }
}

impl Display for MeasurementType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MeasurementType::Prove => write!(f, "prove"),
            MeasurementType::Exec => write!(f, "exec"),
        }
    }
}

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
