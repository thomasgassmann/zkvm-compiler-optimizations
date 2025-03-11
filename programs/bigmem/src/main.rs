#![no_main]

/*
    Adapted from https://github.com/a16z/zkvm-benchmarks
*/

use core::hint::black_box;

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

fn main() {
    #[cfg(feature = "sp1")]
    let value = sp1_zkvm::io::read::<u32>();
    #[cfg(feature = "risc0")]
    let value = risc0_zkvm::guest::env::read::<u32>();

    let array = [value; 12800];
    black_box(array);
    let result = array[1600];

    #[cfg(feature = "sp1")]
    sp1_zkvm::io::commit(&result);
    #[cfg(feature = "risc0")]
    risc0_zkvm::guest::env::commit(&result);
}
