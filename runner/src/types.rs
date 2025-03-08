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
    Bigmem
}

/// An identifier used to select the prover to evaluate.
#[derive(clap::ValueEnum, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ProverId {
    #[serde(rename = "risc0")]
    Risc0,
    #[serde(rename = "sp1")]
    SP1,
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub profiles: HashMap<String, Profile>,
    pub zkvms: Vec<ProverId>,
    pub programs: ProgramConfig,
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
