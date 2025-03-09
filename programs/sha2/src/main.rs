#![no_main]

use sha2::{Digest, Sha256};
extern crate alloc;

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    #[cfg(feature = "sp1")]
    let input: Vec<u8> = sp1_zkvm::io::read();
    #[cfg(feature = "risc0")]
    let input: Vec<u8> = risc0_zkvm::guest::env::read();

    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();

    #[cfg(feature = "sp1")]
    sp1_zkvm::io::commit::<[u8; 32]>(&result.into());
    #[cfg(feature = "risc0")]
    risc0_zkvm::guest::env::commit::<[u8; 32]>(&result.into());
}
