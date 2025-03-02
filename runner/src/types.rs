use std::fmt::{Display, Formatter};

/// An identifier used to select the program to evaluate.
#[derive(clap::ValueEnum, Clone, PartialEq)]
#[clap(rename_all = "kebab-case")]
pub enum ProgramId {
    LoopSum,
    Factorial,
    Sha256,
    RustTests,
    Keccak256
}

/// An identifier used to select the prover to evaluate.
#[derive(clap::ValueEnum, Clone, PartialEq)]
pub enum ProverId {
    Risc0,
    SP1,
}

impl Display for ProgramId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramId::LoopSum => write!(f, "loop-sum"),
            ProgramId::Factorial => write!(f, "factorial"),
            ProgramId::Sha256 => write!(f, "sha256"),
            ProgramId::RustTests => write!(f, "rust-tests"),
            ProgramId::Keccak256 => write!(f, "keccak256")
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
