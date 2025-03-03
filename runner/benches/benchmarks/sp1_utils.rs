use runner::types::ProgramId;
use sp1_prover::components::CpuProverComponents;
use sp1_sdk::{SP1Context, SP1Prover};

use crate::benchmarks::input::get_sp1_stdin;

pub fn exec_sp1(elf: &[u8], program: &ProgramId) {
    let stdin = get_sp1_stdin(program);

    let prover = SP1Prover::<CpuProverComponents>::new();
    let (_, _, _, _) = prover.setup(&elf);

    prover.execute(&elf, &stdin, SP1Context::default()).unwrap();
}
