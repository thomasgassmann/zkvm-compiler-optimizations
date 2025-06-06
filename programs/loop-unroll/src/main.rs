#![no_main]

use loopunroll::unroll;

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    let data: [u64; 32] = [
        10, 20, 30, 40,
        50, 60, 70, 80,
        90, 100, 110, 120,
        130, 140, 150, 160,
        170, 180, 190, 200,
        210, 220, 230, 240,
        250, 260, 270, 280,
        290, 300, 310, 320,
    ];
    #[cfg(feature = "risc0")]
    let reps: usize = risc0_zkvm::guest::env::read();
    #[cfg(feature = "sp1")]
    let reps: usize = sp1_zkvm::io::read();
    let res = unroll(&data, reps);
    core::hint::black_box(res);
}
