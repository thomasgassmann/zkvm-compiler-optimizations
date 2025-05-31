#![no_main]

use merkle::exec;

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    #[cfg(feature = "risc0")]
    let input: Vec<String> = risc0_zkvm::guest::env::read();
    #[cfg(feature = "sp1")]
    let input: Vec<String> = sp1_zkvm::io::read();
    #[cfg(feature = "risc0")]
    let range: std::ops::Range<usize> = risc0_zkvm::guest::env::read();
    #[cfg(feature = "sp1")]
    let range: std::ops::Range<usize> = sp1_zkvm::io::read();
    exec(input, range).unwrap();
}
