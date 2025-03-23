#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

use polybench_rs::stencils::jacobi_2d::bench;

fn bench_and_print<const N: usize, const TSTEPS: usize>() {
    bench::<N, TSTEPS>();
}

fn main() {
    bench_and_print::<10, 5>();
    // bench_and_print::<325, 125>();
    // bench_and_print::<650, 250>();
    // bench_and_print::<1300, 500>();
}
