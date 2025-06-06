pub fn unroll(a: &[u64; 32], reps: usize) -> u64 {
    let mut sum = 0u64;
    for _ in 0..reps {
        sum = 0;
        for i in 0..32 {
            sum += a[i];
        }

        std::hint::black_box(sum);
    }

    sum
}

#[no_mangle]
#[cfg(feature = "x86")]
pub extern "C" fn main_core(reps: usize) -> () {
    let data: [u64; 32] = [
        10, 20, 30, 40,
        50, 60, 70, 80,
        90, 100, 110, 120,
        130, 140, 150, 160,
        170, 180, 190, 200,
        210, 220, 230, 240,
        250, 260, 270, 280,
        290, 300, 310, 320,
    ];
    let res = unroll(&data, reps);
    core::hint::black_box(res);
}
