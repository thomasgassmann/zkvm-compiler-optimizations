#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(npbcg::main_core);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(npbcg::main_core);
