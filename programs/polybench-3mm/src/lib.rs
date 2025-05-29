use polybench_rs::linear_algebra::kernels::_3mm::bench;

fn bench_and_print<
    const A: usize,
    const B: usize,
    const C: usize,
    const D: usize,
    const E: usize,
>() {
    bench::<A, B, C, D, E>();
}

#[no_mangle]
pub fn main_core() {
    bench_and_print::<8, 9, 10, 11, 12>();
    // bench_and_print::<200, 225, 250, 275, 300>();
    // bench_and_print::<400, 450, 500, 550, 600>();
    // bench_and_print::<800, 900, 1000, 1100, 1200>();
}
