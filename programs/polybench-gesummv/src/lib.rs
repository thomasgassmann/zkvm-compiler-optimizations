use polybench_rs::linear_algebra::blas::gesummv::bench;

fn bench_and_print<const N: usize>() {
    bench::<N>();
}

#[no_mangle]
pub fn main_core() {
    bench_and_print::<40>();
    // bench_and_print::<5000>();
    // bench_and_print::<10000>();
    // bench_and_print::<20000>();
}
