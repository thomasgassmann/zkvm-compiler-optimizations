#[cfg(all(feature = "nodiv", feature = "x86"))]
#[inline(never)]
pub fn div_by_10(x: u32) -> u32 {
    x / 10
}

#[cfg(all(feature = "div", feature = "x86"))]
#[inline(never)]
pub fn div_by_10(mut x: u32) -> u32 {
    unsafe {
        core::arch::asm!(
            "xor edx, edx", // zero-extend divisor
            "div ecx",
            inlateout("eax") x, 
            in("ecx")  10u32,
            lateout("edx") _, // we clobber edx
            options(nomem, pure, nostack),
        );
    }
    x
}

#[cfg(feature = "x86")]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn main_core(data: Vec<u32>) {
    for _ in 0..100 {
        for i in 0..data.len() {
            let res = div_by_10(data[i]);
            core::hint::black_box(res);
        }
    }
}
