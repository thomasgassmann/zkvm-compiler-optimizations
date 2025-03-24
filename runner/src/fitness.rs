use crate::{bench::{risc0_utils::get_risc0_stats, sp1_utils::get_sp1_stats}, types::{ProgramId, ProverId}};

pub fn eval_fitness(elf: &[u8], program: &ProgramId, zkvm: &ProverId) -> () {
    let stats = match zkvm {
        ProverId::Risc0 => get_risc0_stats(elf, program),
        ProverId::SP1 => get_sp1_stats(elf, program)
    };
    println!("{}", stats.cycle_count);
}
