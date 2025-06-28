#![no_main]

// use std::arch::asm;
use core::hint::black_box;

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

// #[cfg(feature = "div")]
// #[inline(never)]
// fn div(a: i32) -> i32 {
//     let result: i32;
//     unsafe {
//         asm!(
//             "li t0, 8",
//             "div {0}, {0}, t0",
//             inout(reg) a => result,
//         );
//     }
//     result
// }

// #[cfg(feature = "shift")]
// #[inline(never)]
// fn div(a: i32) -> i32 {
//     let result: i32;
//     unsafe {
//         asm!(
//             "srai a1, {0}, 31",
//             "srli a1, a1, 29",
//             "add {0}, {0}, a1",
//             "srai {0}, {0}, 3",
//             inout(reg) a => result
//         );
//     }
//     result
// }

#[cfg(feature = "fill-fused")]
#[inline(never)]
fn fill(a: &mut [i32], b: &mut [i32]) {
    for i in 0..a.len() {
        unsafe {
            *a.get_unchecked_mut(i) = 1;
            *b.get_unchecked_mut(i) = 2;
        }
    }
}

#[cfg(feature = "fill-split")]
#[inline(never)]
fn fill(a: &mut [i32], b: &mut [i32]) {
    for x in a.iter_mut() { *x = 1; }
    for x in b.iter_mut() { *x = 2; }
}

fn main() {
    #[cfg(feature = "risc0")]
    let mut data: (Vec<i32>, Vec<i32>) = risc0_zkvm::guest::env::read();
    #[cfg(feature = "sp1")]
    let mut data: (Vec<i32>, Vec<i32>) = sp1_zkvm::io::read();
    fill(black_box(&mut data.0), black_box(&mut data.1));
}
