use polybench_rs::linear_algebra::blas::gemm::bench;

fn bench_and_print<const NI: usize, const NJ: usize, const NK: usize>() {
    bench::<NI, NJ, NK>();
}

#[no_mangle]
pub fn main_core() {
    bench_and_print::<10, 11, 12>();
    // bench_and_print::<250, 275, 300>();
    // bench_and_print::<500, 550, 600>();
    // bench_and_print::<1000, 1100, 1200>();
}
