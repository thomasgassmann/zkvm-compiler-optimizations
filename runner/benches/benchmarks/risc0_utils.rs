use crate::benchmarks::input::set_risc0_input;
use risc0_zkvm::{ExecutorEnv, ExecutorImpl};
use runner::{
    types::{ProgramId, ProverId},
    utils::read_elf,
};

pub fn get_risc0_executor(program: &ProgramId) -> ExecutorImpl<'_> {
    let elf = read_elf(program, &ProverId::Risc0);

    let mut builder = ExecutorEnv::builder();
    set_risc0_input(program, &mut builder);
    let env = builder.build().unwrap();

    ExecutorImpl::from_elf(env, &elf).unwrap()
}
