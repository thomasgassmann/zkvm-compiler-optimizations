#![no_main]

use std::hint::black_box;

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    #[cfg(feature = "risc0")]
    let n: u32 = risc0_zkvm::guest::env::read();
    #[cfg(feature = "sp1")]
    let n: u32 = sp1_zkvm::io::read();
    let result = fibonacci::fibonacci(n);
    println!("result: {}", result);
    #[cfg(feature = "sp1")]
    sp1_zkvm::io::commit(&result);
    #[cfg(feature = "risc0")]
    risc0_zkvm::guest::env::commit(&result);
}
