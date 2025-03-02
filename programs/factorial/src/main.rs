#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

pub fn factorial(num: u128) -> u128 {
    (1..=num).product()
}

fn main() {
    #[cfg(feature = "risc0")]
    let n: u32 = risc0_zkvm::guest::env::read();
    #[cfg(feature = "sp1")]
    let n: u32 = sp1_zkvm::io::read();

    for _i in 0..n {
        let _ = factorial(20);
    }
}
