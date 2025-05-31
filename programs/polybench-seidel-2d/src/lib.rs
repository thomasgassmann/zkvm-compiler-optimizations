use polybench_rs::stencils::seidel_2d::bench;

fn bench_and_print<const N: usize, const TSTEPS: usize>() {
    bench::<N, TSTEPS>();
}

#[no_mangle]
pub fn main_core() {
    bench_and_print::<20, 5>();
    // bench_and_print::<500, 125>();
    // bench_and_print::<1000, 250>();
    // bench_and_print::<2000, 500>();
}
