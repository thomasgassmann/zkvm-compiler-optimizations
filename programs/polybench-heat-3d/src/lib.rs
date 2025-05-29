use polybench_rs::stencils::heat_3d::bench;

fn bench_and_print<const N: usize, const TSTEPS: usize>() {
    bench::<N, TSTEPS>();
}

#[no_mangle]
pub fn main_core() {
    bench_and_print::<5, 10>();
    // bench_and_print::<30, 125>();
    // bench_and_print::<60, 250>();
    // bench_and_print::<120, 500>();
}
