use polybench_rs::linear_algebra::kernels::atax::bench;

fn bench_and_print<const M: usize, const N: usize>() {
    bench::<M, N>();
}

#[no_mangle]
pub fn main_core() {
    bench_and_print::<19, 21>();
    // bench_and_print::<475, 525>();
    // bench_and_print::<950, 1050>();
    // bench_and_print::<1900, 2100>();
}
