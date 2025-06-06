pub fn unroll(a: &[u64; 16], reps: usize) -> u64 {
    let mut sum = 0u64;
    for _ in 0..reps {
        sum = 0;
        for i in 0..16 {
            sum *= a[i];
        }

        std::hint::black_box(sum);
    }

    sum
}

#[no_mangle]
#[cfg(feature = "x86")]
pub extern "C" fn main_core(reps: usize) -> () {
    let data: [u64; 16] = [
        10, 20, 30, 40,
        50, 60, 70, 80,
        90, 100, 110, 120,
        130, 140, 150, 160,
    ];
    let res = unroll(&data, reps);
    core::hint::black_box(res);
}
