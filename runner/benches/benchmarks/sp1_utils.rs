use runner::types::ProgramId;
use sp1_prover::components::CpuProverComponents;
use sp1_sdk::{SP1Context, SP1Prover, SP1Stdin};

use crate::benchmarks::input::get_sp1_stdin;

pub fn exec_sp1_prepare(
    elf: &[u8],
    program: &ProgramId,
) -> (SP1Stdin, SP1Prover<CpuProverComponents>) {
    let stdin = get_sp1_stdin(program);

    let prover = SP1Prover::<CpuProverComponents>::new();
    let (_, _, _, _) = prover.setup(&elf);
    (stdin, prover)
}

pub fn exec_sp1(stdin: SP1Stdin, prover: SP1Prover<CpuProverComponents>, elf: &[u8]) {
    prover.execute(&elf, &stdin, SP1Context::default()).unwrap();
}
