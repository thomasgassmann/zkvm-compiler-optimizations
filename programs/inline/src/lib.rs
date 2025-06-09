#[inline(always)]
pub fn work_inlined(x: u64) -> u64 {
    let mut sum = x;
    for i in 0..100 {
        sum = sum.wrapping_mul(31).wrapping_add(i);
    }
    sum
}

#[inline(never)]
pub fn work_non_inlined(x: u64) -> u64 {
    let mut sum = x;
    for i in 0..100 {
        sum = sum.wrapping_mul(31).wrapping_add(i);
    }
    sum
}

#[cfg(feature = "x86")]
#[no_mangle]
pub extern "C" fn main_core(n: u32) {
    for i in 0..n {
        #[cfg(feature = "inline")]
        let _ = work_inlined(i as u64);
        #[cfg(not(feature = "inline"))]
        let _ = work_non_inlined(i as u64);
    }
}
