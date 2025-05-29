use polybench_rs::stencils::fdtd_2d::bench;

fn bench_and_print<const NX: usize, const NY: usize, const TMAX: usize>() {
    bench::<NX, NY, TMAX>();
}

#[no_mangle]
pub fn main_core() {
    bench_and_print::<10, 12, 5>();
    // bench_and_print::<250, 300, 125>();
    // bench_and_print::<500, 600, 250>();
    // bench_and_print::<1000, 1200, 500>();
}
