#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(npbis::main_core);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(npbis::main_core);
