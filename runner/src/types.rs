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
    C
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
    pub prepopulate_passes: bool
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
            ProgramId::C => write!(f, "c"),
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
