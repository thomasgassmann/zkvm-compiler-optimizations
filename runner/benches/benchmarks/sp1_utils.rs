use runner::{input::get_sp1_stdin, types::ProgramId};
use sp1_core_executor::Program;
use sp1_prover::components::CpuProverComponents;
use sp1_sdk::{SP1Context, SP1Prover, SP1Stdin};
use sp1_stark::{baby_bear_poseidon2::BabyBearPoseidon2, SP1ProverOpts, StarkProvingKey};

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

pub fn prove_core_sp1_prepare(
    elf: &[u8],
    program: &ProgramId,
) -> (
    SP1Stdin,
    SP1Prover<CpuProverComponents>,
    Program,
    StarkProvingKey<BabyBearPoseidon2>,
    SP1ProverOpts,
) {
    let stdin = get_sp1_stdin(program);
    let prover = SP1Prover::<CpuProverComponents>::new();
    let (_, pk_d, program, _) = prover.setup(&elf);
    let opts = SP1ProverOpts::auto();
    (stdin, prover, program, pk_d, opts)
}

pub fn prove_core_sp1(
    stdin: SP1Stdin,
    prover: SP1Prover<CpuProverComponents>,
    program: Program,
    proving_key: StarkProvingKey<BabyBearPoseidon2>,
    opts: SP1ProverOpts,
) {
    prover
        .prove_core(&proving_key, program, &stdin, opts, SP1Context::default())
        .unwrap();
}
