use polybench_rs::stencils::adi::bench;

fn bench_and_print<const N: usize, const TSTEPS: usize>() {
    bench::<N, TSTEPS>();
}

#[no_mangle]
pub fn main_core() {
    bench_and_print::<12, 6>();
    // bench_and_print::<250, 125>();
    // bench_and_print::<500, 250>();
    // bench_and_print::<1000, 500>();
}
