use crate::benchmarks::input::set_risc0_input;
use risc0_zkvm::{ExecutorEnv, ExecutorImpl};
use runner::types::ProgramId;


pub fn exec_risc0_setup(elf: &[u8], program: &ProgramId) -> ExecutorImpl<'static> {
    let mut builder = ExecutorEnv::builder();
    builder.stdout(std::io::sink());
    set_risc0_input(program, &mut builder);
    let env = builder.build();
    ExecutorImpl::from_elf(env.unwrap(), elf).unwrap()
}

pub fn exec_risc0(mut p: ExecutorImpl<'static>) {
    p.run().unwrap();
}
