#![no_main]

use std::arch::asm;

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

#[cfg(feature = "raw")]
#[inline(never)]
fn div(a: i32) -> i32 {
    return a / 8;
}

#[cfg(feature = "div")]
#[inline(never)]
fn div(a: i32) -> i32 {
    let result: i32;
    unsafe {
        asm!(
            "li t0, 8",
            "div {0}, {0}, t0",
            inout(reg) a => result,
        );
    }
    result
}


#[cfg(feature = "shift")]
#[inline(never)]
fn div(a: i32) -> i32 {
    let result: i32;
    unsafe {
        asm!(
            "srai a1, {0}, 31",
            "srli a1, a1, 29",
            "add {0}, {0}, a1",
            "srai {0}, {0}, 3",
            inout(reg) a => result
        );
    }
    result
}

fn main() {
    #[cfg(feature = "risc0")]
    let data: Vec<i32> = risc0_zkvm::guest::env::read();
    #[cfg(feature = "sp1")]
    let data: Vec<i32> = sp1_zkvm::io::read();

    for _ in 0..1000 {
        for i in 0..data.len() {
            let res = div(data[i]);
            core::hint::black_box(res);
        }
    }
}
