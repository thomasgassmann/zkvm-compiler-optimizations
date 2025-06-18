#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

#[inline(never)]
fn abs_i32_branchy(x: i32) -> i32 {
    if x < 0 {
        x.wrapping_neg()
    } else {
        x
    }
}

pub fn main() {
    #[cfg(feature = "risc0")]
    let data: Vec<i32> = risc0_zkvm::guest::env::read();
    #[cfg(feature = "sp1")]
    let data: Vec<i32> = sp1_zkvm::io::read();
    for _ in 0..1000 {
        for i in 0..data.len() {
            let _ = abs_i32_branchy(data[i]);
        }
    }
}
