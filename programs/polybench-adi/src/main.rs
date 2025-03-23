#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

use polybench_rs::stencils::adi::bench;

fn bench_and_print<const N: usize, const TSTEPS: usize>() {
    bench::<N, TSTEPS>();
}

fn main() {
    bench_and_print::<12, 6>();
    // bench_and_print::<250, 125>();
    // bench_and_print::<500, 250>();
    // bench_and_print::<1000, 500>();
}
