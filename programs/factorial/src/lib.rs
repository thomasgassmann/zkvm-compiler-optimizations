pub fn factorial(num: u128) -> u128 {
    (1..=num).product()
}

#[no_mangle]
pub extern "C" fn main_core(n: u32) {
    for _i in 0..n {
        let res = factorial(20);
        core::hint::black_box(res);
    }
}
