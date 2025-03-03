use core::time;
use std::fs;
use std::{env, time::Instant};

use crate::types::{ProgramId, ProverId};

pub fn time_operation<T, F: FnOnce() -> T>(operation: F) -> (T, time::Duration) {
    let start = Instant::now();
    let result = operation();
    let duration = start.elapsed();
    (result, duration)
}

pub fn read_elf(program: &ProgramId, prover: &ProverId) -> Vec<u8> {
    let elf_path = get_elf(program, prover);
    fs::read(elf_path).unwrap()
}

pub fn get_elf(program: &ProgramId, prover: &ProverId) -> String {
    let mut program_dir = program.to_string();

    match program {
        ProgramId::Keccak256 => {
            program_dir.push('-');
            program_dir.push_str(&prover.to_string());
        }
        _ => {}
    };

    let current_dir = env::current_dir().expect("Failed to get current working directory");
    return match prover {
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
}
