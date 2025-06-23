#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

#[cfg(feature = "nodiv")]
#[inline(never)]
pub fn div_by_10(x: u32) -> u32 {
    x / 10
}

#[cfg(feature = "div")]
#[inline(never)]
pub fn div_by_10(mut x: u32) -> u32 {
    unsafe {
        core::arch::asm!(
            "li  t0, 10",
            "div a0, a0, t0",
            inout("a0") x,
            options(nomem, nostack, pure)
        );
    }
    x
}

fn main() {
    #[cfg(feature = "risc0")]
    let data: Vec<u32> = risc0_zkvm::guest::env::read();
    #[cfg(feature = "sp1")]
    let data: Vec<u32> = sp1_zkvm::io::read();
    for _ in 0..100 {
        for i in 0..data.len() {
            let res = div_by_10(data[i]);
            core::hint::black_box(res);
        }
    }
}
