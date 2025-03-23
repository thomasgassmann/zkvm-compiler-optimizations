#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

use polybench_rs::linear_algebra::kernels::atax::bench;

fn bench_and_print<const M: usize, const N: usize>() {
    bench::<M, N>();
}

fn main() {
    bench_and_print::<19, 21>();
    // bench_and_print::<475, 525>();
    // bench_and_print::<950, 1050>();
    // bench_and_print::<1900, 2100>();
}
