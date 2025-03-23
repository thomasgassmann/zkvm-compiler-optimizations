#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

use polybench_rs::linear_algebra::kernels::doitgen::bench;

fn bench_and_print<const NP: usize, const NQ: usize, const NR: usize>() {
    bench::<NP, NQ, NR>();
}

fn main() {
    bench_and_print::<6, 4, 5>();
    // bench_and_print::<35, 37, 40>();
    // bench_and_print::<70, 75, 80>();
    // bench_and_print::<140, 150, 160>();
}
