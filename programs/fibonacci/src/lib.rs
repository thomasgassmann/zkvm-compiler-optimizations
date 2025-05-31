pub fn fibonacci(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let sum = (a + b) % 7919; // Mod to avoid overflow
        a = b;
        b = sum;
    }
    b
}

#[cfg(feature = "x86")]
#[no_mangle]
pub extern "C" fn main_core(n: u32) {
    let result = fibonacci(n);
    core::hint::black_box(result);
}
