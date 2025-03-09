#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

fn main() {
    #[cfg(feature = "risc0")]
    let n: u32 = risc0_zkvm::guest::env::read();
    #[cfg(feature = "sp1")]
    let n: u32 = sp1_zkvm::io::read();

    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut sum: u128;
    for _ in 1..n {
        sum = a + b;
        a = b;
        b = sum;
    }

    #[cfg(feature = "sp1")]
    sp1_zkvm::io::commit(&b);
    #[cfg(feature = "risc0")]
    risc0_zkvm::guest::env::commit(&b);
}
