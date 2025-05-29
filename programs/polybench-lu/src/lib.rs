use polybench_rs::linear_algebra::solvers::lu::bench;

fn bench_and_print<const N: usize>() {
    bench::<N>();
}

#[no_mangle]
pub fn main_core() {
    bench_and_print::<15>();
    // bench_and_print::<500>();
    // bench_and_print::<1000>();
    // bench_and_print::<2000>();
}
