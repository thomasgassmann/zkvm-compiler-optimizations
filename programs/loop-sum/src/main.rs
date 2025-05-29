#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    #[cfg(feature = "risc0")]
    let data: Vec<i32> = risc0_zkvm::guest::env::read();
    #[cfg(feature = "sp1")]
    let data: Vec<i32> = sp1_zkvm::io::read();

    let result = sum_array(&data);
    println!("result: {}", result);
}
