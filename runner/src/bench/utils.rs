use std::{fs, path::PathBuf};

use crate::{types::MeasurementType, utils::read_elf};

use super::super::types::{ProgramId, ProverId};
use k256::sha2;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ElfStats {
    pub cycle_count: u64,
    pub size: usize,
    pub hash: String,
}

#[derive(Debug, Serialize)]
pub struct ElfRef {
    pub same_as: str,
}

static BASLINE_PROFILE: &'static str = "baseline";

pub fn is_same_as_baseline(program: &ProgramId, prover: &ProverId, profile: &String) -> bool {
    if profile == &String::from(BASLINE_PROFILE) {
        return false;
    }

    let elf = read_elf(program, prover, profile);
    let baseline_elf = read_elf(program, prover, &String::from(BASLINE_PROFILE));
    elf.len() == baseline_elf.len()
        && elf
            .iter()
            .zip(baseline_elf.iter())
            .all(|(a, b)| a == b)
}

pub fn get_elf_hash(elf: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(elf);
    format!("{:x}", hasher.finalize())
}

pub fn has_previously_run(
    program: &ProgramId,
    zkvm: &ProverId,
    measurement: &MeasurementType,
) -> bool {
    let mut path: PathBuf = get_criterion_dir();
    path.push(format!("{}-{}-{}", program, zkvm, measurement));
    path.exists()
}

pub fn get_criterion_dir() -> PathBuf {
    let mut path = PathBuf::from(std::env::current_dir().unwrap());
    path.push("target/criterion");
    fs::create_dir_all(&path).unwrap();
    path
}

pub fn get_criterion_meta_dir() -> PathBuf {
    let mut path = get_criterion_dir();
    path.push("meta");
    fs::create_dir_all(&path).unwrap();
    path
}

pub fn get_elf_stats_path(program: &ProgramId, zkvm: &ProverId, profile: &String) -> PathBuf {
    let mut path = get_criterion_meta_dir();
    path.push(program.to_string());
    path.push(zkvm.to_string());
    fs::create_dir_all(&path).unwrap();
    path.push(format!("{}.json", profile));
    path
}

pub fn write_elf_stats(program: &ProgramId, zkvm: &ProverId, profile: &String, stats: &ElfStats) {
    println!("Writing elf stats to {:?}", profile);
    let path = get_elf_stats_path(program, zkvm, profile);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let file = std::fs::File::create(path).unwrap();
    serde_json::to_writer_pretty(file, stats).unwrap();
}
