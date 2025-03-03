use risc0_zkvm::{ExecutorEnv, ExecutorImpl};
use runner::{input::set_risc0_input, types::ProgramId};


pub fn exec_risc0_setup<'a>(elf: &'a [u8], program: &'a ProgramId) -> ExecutorImpl<'a> {
    let mut builder = ExecutorEnv::builder();
    builder.stdout(std::io::sink());
    set_risc0_input(program, &mut builder);
    let env = builder.build();
    ExecutorImpl::from_elf(env.unwrap(), elf).unwrap()
}

pub fn exec_risc0(mut p: ExecutorImpl<'_>) {
    p.run().unwrap();
}
