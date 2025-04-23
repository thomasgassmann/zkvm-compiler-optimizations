use std::fmt::{Display, Formatter};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// An identifier used to select the program to evaluate.
#[derive(clap::ValueEnum, Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
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
    #[serde(rename = "polybench-2mm")]
    #[clap(name = "polybench-2mm")]
    Polybench2mm,
    #[serde(rename = "polybench-3mm")]
    #[clap(name = "polybench-3mm")]
    Polybench3mm,
    PolybenchAdi,
    PolybenchAtax,
    PolybenchBicg,
    PolybenchCholesky,
    PolybenchCorrelation,
    PolybenchCovariance,
    PolybenchDeriche,
    PolybenchDoitgen,
    PolybenchDurbin,
    #[serde(rename = "polybench-fdtd-2d")]
    #[clap(name = "polybench-fdtd-2d")]
    PolybenchFdtd2d,
    PolybenchFloydWarshall,
    PolybenchGemm,
    PolybenchGemver,
    PolybenchGesummv,
    PolybenchGramschmidt,
    #[serde(rename = "polybench-heat-3d")]
    #[clap(name = "polybench-heat-3d")]
    PolybenchHeat3d,
    #[serde(rename = "polybench-jacobi-1d")]
    #[clap(name = "polybench-jacobi-1d")]
    PolybenchJacobi1d,
    #[serde(rename = "polybench-jacobi-2d")]
    #[clap(name = "polybench-jacobi-2d")]
    PolybenchJacobi2d,
    PolybenchLu,
    PolybenchLudcmp,
    PolybenchMvt,
    PolybenchNussinov,
    #[serde(rename = "polybench-seidel-2d")]
    #[clap(name = "polybench-seidel-2d")]
    PolybenchSeidel2d,
    PolybenchSymm,
    PolybenchSyr2k,
    PolybenchSyrk,
    PolybenchTrisolv,
    PolybenchTrmm,
    Merkle,
    EcdsaVerify,
    EddsaVerify,
    #[serde(rename = "spec-619")]
    #[clap(name = "spec-619")]
    Spec619,
    #[serde(rename = "spec-605")]
    #[clap(name = "spec-605")]
    Spec605,
    NpbBt,
    NpbCg,
    NpbEp,
    NpbFt,
    NpbIs,
    NpbLu,
    NpbMg,
    NpbSp,
    #[serde(rename = "spec-631")]
    #[clap(name = "spec-631")]
    Spec631,
}

#[derive(clap::ValueEnum, Clone, PartialEq, Debug, Serialize, Deserialize, Hash)]
pub enum TuneMetric {
    CycleCount,
    ProveTime,
}

/// An identifier used to select the prover to evaluate.
#[derive(clap::ValueEnum, Clone, PartialEq, Debug, Serialize, Deserialize, Hash)]
pub enum ProverId {
    #[serde(rename = "risc0")]
    Risc0,
    #[serde(rename = "sp1")]
    SP1,
}

#[derive(clap::ValueEnum, Clone, Debug, Serialize, Deserialize, Hash, PartialEq)]
pub enum MeasurementType {
    #[serde(rename = "prove")]
    Prove,
    #[serde(rename = "exec")]
    Exec,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramConfig {
    #[serde(default)]
    pub specific: bool,
    pub groups: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Hash)]
pub struct Profile {
    pub rustflags: String,
    pub passes: Vec<String>,
    pub prepopulate_passes: bool,
    #[serde(default)]
    pub lower_atomic_before: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub profiles: IndexMap<String, Profile>,
    pub zkvms: Vec<ProverId>,
    pub programs: IndexMap<ProgramId, ProgramConfig>,
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
            ProgramId::Polybench2mm => write!(f, "polybench-2mm"),
            ProgramId::Polybench3mm => write!(f, "polybench-3mm"),
            ProgramId::PolybenchAdi => write!(f, "polybench-adi"),
            ProgramId::PolybenchAtax => write!(f, "polybench-atax"),
            ProgramId::PolybenchBicg => write!(f, "polybench-bicg"),
            ProgramId::PolybenchCholesky => write!(f, "polybench-cholesky"),
            ProgramId::PolybenchCorrelation => write!(f, "polybench-correlation"),
            ProgramId::PolybenchCovariance => write!(f, "polybench-covariance"),
            ProgramId::PolybenchDeriche => write!(f, "polybench-deriche"),
            ProgramId::PolybenchDoitgen => write!(f, "polybench-doitgen"),
            ProgramId::PolybenchDurbin => write!(f, "polybench-durbin"),
            ProgramId::PolybenchFdtd2d => write!(f, "polybench-fdtd-2d"),
            ProgramId::PolybenchFloydWarshall => write!(f, "polybench-floyd-warshall"),
            ProgramId::PolybenchGemm => write!(f, "polybench-gemm"),
            ProgramId::PolybenchGemver => write!(f, "polybench-gemver"),
            ProgramId::PolybenchGesummv => write!(f, "polybench-gesummv"),
            ProgramId::PolybenchGramschmidt => write!(f, "polybench-gramschmidt"),
            ProgramId::PolybenchHeat3d => write!(f, "polybench-heat-3d"),
            ProgramId::PolybenchJacobi1d => write!(f, "polybench-jacobi-1d"),
            ProgramId::PolybenchJacobi2d => write!(f, "polybench-jacobi-2d"),
            ProgramId::PolybenchLu => write!(f, "polybench-lu"),
            ProgramId::PolybenchLudcmp => write!(f, "polybench-ludcmp"),
            ProgramId::PolybenchMvt => write!(f, "polybench-mvt"),
            ProgramId::PolybenchNussinov => write!(f, "polybench-nussinov"),
            ProgramId::PolybenchSeidel2d => write!(f, "polybench-seidel-2d"),
            ProgramId::PolybenchSymm => write!(f, "polybench-symm"),
            ProgramId::PolybenchSyr2k => write!(f, "polybench-syr2k"),
            ProgramId::PolybenchSyrk => write!(f, "polybench-syrk"),
            ProgramId::PolybenchTrisolv => write!(f, "polybench-trisolv"),
            ProgramId::PolybenchTrmm => write!(f, "polybench-trmm"),
            ProgramId::Merkle => write!(f, "merkle"),
            ProgramId::EcdsaVerify => write!(f, "ecdsa-verify"),
            ProgramId::EddsaVerify => write!(f, "eddsa-verify"),
            ProgramId::Spec619 => write!(f, "spec-619"),
            ProgramId::Spec605 => write!(f, "spec-605"),
            ProgramId::NpbBt => write!(f, "npb-bt"),
            ProgramId::NpbCg => write!(f, "npb-cg"),
            ProgramId::NpbEp => write!(f, "npb-ep"),
            ProgramId::NpbFt => write!(f, "npb-ft"),
            ProgramId::NpbIs => write!(f, "npb-is"),
            ProgramId::NpbLu => write!(f, "npb-lu"),
            ProgramId::NpbMg => write!(f, "npb-mg"),
            ProgramId::NpbSp => write!(f, "npb-sp"),
            ProgramId::Spec631 => write!(f, "spec-631"),
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
