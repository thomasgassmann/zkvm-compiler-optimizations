use polybench_rs::datamining::correlation::bench;

fn bench_and_print<const M: usize, const N: usize>() {
    bench::<M, N>();
}

#[no_mangle]
pub fn main_core() {
    bench_and_print::<12, 14>();
    // bench_and_print::<300, 350>();
    // bench_and_print::<600, 700>();
    // bench_and_print::<1200, 1400>();
}
