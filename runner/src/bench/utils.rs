use std::path::PathBuf;

use runner::types::{ProgramId, ProverId};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ElfStats {
    pub cycle_count: u64,
}

pub fn get_criterion_dir(program: &ProgramId, zkvm: &ProverId) -> PathBuf {
    let mut path = PathBuf::from(std::env::current_dir().unwrap());
    path.push("target/criterion");
    path.push(format!("{}-{}", program, zkvm));
    path
}

pub fn get_elf_stats_path(program: &ProgramId, zkvm: &ProverId, profile: &String) -> PathBuf {
    let mut path = get_criterion_dir(program, zkvm);
    path.push(format!("{}.json", profile));
    path
}

pub fn write_elf_stats(program: &ProgramId, zkvm: &ProverId, profile: &String, stats: &ElfStats) {
    let path = get_elf_stats_path(program, zkvm, profile);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let file = std::fs::File::create(path).unwrap();
    serde_json::to_writer_pretty(file, stats).unwrap();
}
