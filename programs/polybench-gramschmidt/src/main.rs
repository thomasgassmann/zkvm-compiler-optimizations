#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

use polybench_rs::linear_algebra::solvers::gramschmidt::bench;

fn bench_and_print<const M: usize, const N: usize>() {
    bench::<M, N>();
}

fn main() {
    bench_and_print::<10, 12>();
    // bench_and_print::<250, 300>();
    // bench_and_print::<500, 600>();
    // bench_and_print::<1000, 1200>();
}
