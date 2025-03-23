#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

use polybench_rs::linear_algebra::solvers::trisolv::bench;

fn bench_and_print<const N: usize>() {
    bench::<N>();
}

fn main() {
    bench_and_print::<20>();
    // bench_and_print::<500>();
    // bench_and_print::<1000>();
    // bench_and_print::<2000>();
}
