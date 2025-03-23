#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

use polybench_rs::linear_algebra::blas::gesummv::bench;

fn bench_and_print<const N: usize>() {
    bench::<N>();
}

fn main() {
    bench_and_print::<40>();
    // bench_and_print::<5000>();
    // bench_and_print::<10000>();
    // bench_and_print::<20000>();
}
