#[inline(never)]
pub fn sum_array(arr: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for &val in arr.iter() {
        sum += val;
    }
    sum
}

#[no_mangle]
#[cfg(feature = "x86")]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn main_core(data: Vec<i32>) -> () {
    let result = sum_array(&data);
    core::hint::black_box(&result);
}
