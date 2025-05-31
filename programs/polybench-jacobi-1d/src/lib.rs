use polybench_rs::stencils::jacobi_1d::bench;

fn bench_and_print<const N: usize, const TSTEPS: usize>() {
    bench::<N, TSTEPS>();
}

#[no_mangle]
pub fn main_core() {
    bench_and_print::<20, 100>();
    // bench_and_print::<5000, 125>();
    // bench_and_print::<10000, 250>();
    // bench_and_print::<20000, 500>();
}
