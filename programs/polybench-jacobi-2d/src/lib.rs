use polybench_rs::stencils::jacobi_2d::bench;

fn bench_and_print<const N: usize, const TSTEPS: usize>() {
    bench::<N, TSTEPS>();
}

#[no_mangle]
pub fn main_core() {
    bench_and_print::<10, 5>();
    // bench_and_print::<325, 125>();
    // bench_and_print::<650, 250>();
    // bench_and_print::<1300, 500>();
}
