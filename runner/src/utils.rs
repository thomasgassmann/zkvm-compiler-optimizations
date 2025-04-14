use core::time;
use std::fs::{self, File};
use std::io::BufReader;
use std::{env, time::Instant};

use serde_json::from_reader;

use crate::types::{Config, ProgramId, ProverId};

pub fn is_gpu_proving() -> bool {
    cfg!(feature = "cuda")
}

pub fn time_operation<T, F: FnOnce() -> T>(operation: F) -> (T, time::Duration) {
    let start = Instant::now();
    let result = operation();
    let duration = start.elapsed();
    (result, duration)
}

pub fn read_config_json() -> Config {
    let file = File::open("config.json").expect("could not read config file");
    let reader = BufReader::new(file);

    from_reader(reader).expect("Failed to parse JSON")
}

pub fn read_elf(program: &ProgramId, prover: &ProverId, profile: &String) -> Vec<u8> {
    let elf_path = get_elf(program, prover, profile);
    fs::read(elf_path).unwrap()
}

pub fn get_elf(program: &ProgramId, prover: &ProverId, profile: &String) -> String {
    let current_dir = env::current_dir().expect("Failed to get current working directory");
    current_dir
        .join(format!("bin/{}/{}/{}", program, prover, profile))
        .to_str()
        .expect("Failed to get path")
        .to_string()
}
