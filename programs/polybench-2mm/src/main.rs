#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

use polybench_rs::linear_algebra::kernels::_2mm::bench;

fn bench_and_print<const NI: usize, const NJ: usize, const NK: usize, const NL: usize>() {
    bench::<NI, NJ, NK, NL>();
}

fn main() {
    bench_and_print::<8, 9, 10, 11>();
    // bench_and_print::<200, 225, 250, 275>();
    // bench_and_print::<400, 450, 500, 550>();
    // bench_and_print::<800, 900, 1000, 1100>();
}
