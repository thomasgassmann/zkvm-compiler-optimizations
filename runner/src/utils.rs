use core::time;
use std::fs::{self, File};
use std::io::BufReader;
use std::{env, time::Instant};

use serde_json::from_reader;

use crate::types::{Config, ProgramId, ProverId};

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
    println!("Reading ELF: {}", elf_path);
    fs::read(elf_path).unwrap()
}

pub fn get_elf(program: &ProgramId, prover: &ProverId, profile: &String) -> String {
    let mut program_dir = program.to_string();

    let config = read_config_json();
    if config.programs.specific.contains(program) {
        program_dir.push('-');
        program_dir.push_str(&prover.to_string());
    }

    let current_dir = env::current_dir().expect("Failed to get current working directory");
    let path = match prover {
        ProverId::Risc0 => current_dir
            .join(format!(
                "programs/{}/target/riscv32im-risc0-zkvm-elf/release/{}",
                program_dir, program_dir
            ))
            .to_str()
            .expect("Failed to get path")
            .to_string(),

        ProverId::SP1 => current_dir
            .join(format!(
                "programs/{}/target/riscv32im-succinct-zkvm-elf/release/{}",
                program_dir, program_dir
            ))
            .to_str()
            .expect("Failed to get path")
            .to_string(),
    };

    if profile != "" {
        format!("{}-{}", path, profile)
    } else {
        path
    }
}
