#[inline(never)]
#[cfg(feature = "x86")]
fn abs_i32_branchy(x: i32) -> i32 {
    if x < 0 {
        x.wrapping_neg()
    } else {
        x
    }
}

#[no_mangle]
#[cfg(feature = "x86")]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn main_core(data: Vec<i32>) -> () {
    for _ in 0..1000 {
        for i in 0..data.len() {
            let _ = abs_i32_branchy(data[i]);
        }
    }
}
