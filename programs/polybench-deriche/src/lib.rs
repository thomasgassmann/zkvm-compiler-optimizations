use polybench_rs::medley::deriche::bench;

fn bench_and_print<const H: usize, const W: usize>() {
    bench::<H, W>();
}

#[no_mangle]
pub fn main_core() {
    bench_and_print::<32, 18>();
    // bench_and_print::<1024, 540>();
    // bench_and_print::<2048, 1080>();
    // bench_and_print::<4096, 2160>();
}
