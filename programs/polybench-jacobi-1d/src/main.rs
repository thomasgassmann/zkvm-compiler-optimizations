#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

use polybench_rs::stencils::jacobi_1d::bench;

fn bench_and_print<const N: usize, const TSTEPS: usize>() {
    bench::<N, TSTEPS>();
}

fn main() {
    bench_and_print::<20, 100>();
    // bench_and_print::<5000, 125>();
    // bench_and_print::<10000, 250>();
    // bench_and_print::<20000, 500>();
}
