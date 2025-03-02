use core::time;
use std::{env, time::Instant};

use crate::{types::{ProgramId, ProverId}, EvalArgs};

pub fn time_operation<T, F: FnOnce() -> T>(operation: F) -> (T, time::Duration) {
    let start = Instant::now();
    let result = operation();
    let duration = start.elapsed();
    (result, duration)
}

pub fn get_elf(args: &EvalArgs) -> String {
    let mut program_dir = args.program.to_string();

    match args.program {
        ProgramId::Keccak256 => {
            program_dir.push('-');
            program_dir.push_str(&args.prover.to_string());
        },
        _ => {}
    };

    let current_dir = env::current_dir().expect("Failed to get current working directory");

    return match args.prover {
        ProverId::Risc0 => {
            current_dir.join(format!(
                "programs/{}/target/riscv32im-risc0-zkvm-elf/release/{}",
                program_dir, program_dir
            )).to_str().expect("Failed to get path").to_string()
        }

        ProverId::SP1 => {
            current_dir.join(format!(
                "programs/{}/target/riscv32im-succinct-zkvm-elf/release/{}",
                program_dir, program_dir
            )).to_str().expect("Failed to get path").to_string()
        }
    };
}
