use crate::benchmarks::input::set_risc0_input;
use risc0_zkvm::{ExecutorEnv, ExecutorImpl};
use runner::types::ProgramId;

pub fn exec_risc0(elf: &[u8], program: &ProgramId) {
    let mut builder = ExecutorEnv::builder();
    builder.stdout(std::io::sink());
    set_risc0_input(program, &mut builder);
    let env = builder.build();
    let mut p = ExecutorImpl::from_elf(env.unwrap(), elf).unwrap();
    p.run().unwrap();
}
