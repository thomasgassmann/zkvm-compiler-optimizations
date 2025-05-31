#![no_main]

use sha3::{Digest, Keccak256};
use sha3chain::sha3_hash;

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    #[cfg(feature = "sp1")]
    let input: [u8; 32] = sp1_zkvm::io::read();
    #[cfg(feature = "sp1")]
    let num_iters: u32 = sp1_zkvm::io::read();

    #[cfg(feature = "risc0")]
    let input: [u8; 32] = risc0_zkvm::guest::env::read();
    #[cfg(feature = "risc0")]
    let num_iters: u32 = risc0_zkvm::guest::env::read();

    let hash = sha3_hash!(input, num_iters);

    #[cfg(feature = "sp1")]
    sp1_zkvm::io::commit::<[u8; 32]>(&hash.into());
    #[cfg(feature = "risc0")]
    risc0_zkvm::guest::env::commit::<[u8; 32]>(&hash.into());
}
