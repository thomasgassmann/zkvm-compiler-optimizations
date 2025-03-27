use std::{fs, path::PathBuf};

use crate::types::MeasurementType;

use super::super::types::{ProgramId, ProverId};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ElfStats {
    pub cycle_count: u64,
}

pub fn has_previously_run(program: &ProgramId, zkvm: &ProverId, measurement: &MeasurementType) -> bool {
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
    let path = get_elf_stats_path(program, zkvm, profile);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let file = std::fs::File::create(path).unwrap();
    serde_json::to_writer_pretty(file, stats).unwrap();
}
