#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(npbft::main_core);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(npbft::main_core);
