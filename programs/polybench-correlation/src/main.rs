#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

use polybench_rs::datamining::correlation::bench;

fn bench_and_print<const M: usize, const N: usize>() {
    bench::<M, N>();
}

fn main() {
    bench_and_print::<12, 14>();
    // bench_and_print::<300, 350>();
    // bench_and_print::<600, 700>();
    // bench_and_print::<1200, 1400>();
}
