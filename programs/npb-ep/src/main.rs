#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(npbep::main_core);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(npbep::main_core);
