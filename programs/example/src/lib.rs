#[cfg(feature = "x86")]
use std::arch::asm;
#[cfg(feature = "x86")]
use core::hint::black_box;

#[cfg(all(feature = "div", feature = "x86"))]
#[inline(never)]
pub fn div(a: i32) -> i32 {
    let result: i32;
    let divisor: i32 = 8;
    unsafe {
        asm!(
            "cdq",
            "idiv ecx",
            inout("eax") a => result,
            in("ecx") divisor,
            out("edx") _,
        );
    }
    result
}

#[cfg(all(feature = "shift", feature = "x86"))]
#[inline(never)]
pub fn div(a: i32) -> i32 {
    let result: i32;
    unsafe {
        asm!(
            "lea eax, [rdi + 7]",
            "test edi, edi",
            "cmovns eax, edi",
            "sar eax, 3",
            in("rdi") a,
            out("eax") result,
        );
    }
    result
}

#[cfg(feature = "x86")]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn main_core(value: Vec<i32>) {
    for _ in 0..1000 {
        for i in 0..value.len() {
            let res = div(value[i]);
            black_box(res);
        }
    }
}
