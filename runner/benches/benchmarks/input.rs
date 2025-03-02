use runner::types::ProgramId;
use sp1_sdk::SP1Stdin;

pub fn get_sp1_stdin(program: &ProgramId) -> SP1Stdin {
    let mut stdin = SP1Stdin::new();
    match program {
        ProgramId::Factorial => {
            stdin.write::<u32>(&10);
        }
        ProgramId::Keccak256 => {
            stdin.write(&vec![0u8; 64]);
        }
        _ => {}
    }

    stdin
}

pub fn set_risc0_input(program: &ProgramId, builder: &mut risc0_zkvm::ExecutorEnvBuilder<'_>) {
    match program {
        ProgramId::Factorial => {
            let _ = builder.write::<u32>(&10);
        }
        ProgramId::Keccak256 => {
            let _ = builder.write(&vec![0u8; 64]);
        }
        _ => {}
    }
}
