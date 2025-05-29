#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(npblu::main_core);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(npblu::main_core);
