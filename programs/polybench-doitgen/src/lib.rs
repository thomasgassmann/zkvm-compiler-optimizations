use polybench_rs::linear_algebra::kernels::doitgen::bench;

fn bench_and_print<const NP: usize, const NQ: usize, const NR: usize>() {
    bench::<NP, NQ, NR>();
}

#[no_mangle]
pub fn main_core() {
    bench_and_print::<6, 4, 5>();
    // bench_and_print::<35, 37, 40>();
    // bench_and_print::<70, 75, 80>();
    // bench_and_print::<140, 150, 160>();
}
