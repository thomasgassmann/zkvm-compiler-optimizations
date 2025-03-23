#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

use polybench_rs::stencils::fdtd_2d::bench;

fn bench_and_print<const NX: usize, const NY: usize, const TMAX: usize>() {
    bench::<NX, NY, TMAX>();
}

fn main() {
    bench_and_print::<10, 12, 5>();
    // bench_and_print::<250, 300, 125>();
    // bench_and_print::<500, 600, 250>();
    // bench_and_print::<1000, 1200, 500>();
}
