// #[cfg(feature = "x86")]
// use std::arch::asm;
#[cfg(feature = "x86")]
use core::hint::black_box;

// #[cfg(all(feature = "div", feature = "x86"))]
// #[inline(never)]
// pub fn div(a: i32) -> i32 {
//     let result: i32;
//     let divisor: i32 = 8;
//     unsafe {
//         asm!(
//             "cdq",
//             "idiv ecx",
//             inout("eax") a => result,
//             in("ecx") divisor,
//             out("edx") _,
//         );
//     }
//     result
// }

// #[cfg(all(feature = "shift", feature = "x86"))]
// #[inline(never)]
// pub fn div(a: i32) -> i32 {
//     let result: i32;
//     unsafe {
//         asm!(
//             "lea eax, [rdi + 7]",
//             "test edi, edi",
//             "cmovns eax, edi",
//             "sar eax, 3",
//             in("rdi") a,
//             out("eax") result,
//         );
//     }
//     result
// }

#[cfg(all(feature = "fill-fused", feature = "x86"))]
#[inline(never)]
fn fill(a: &mut [i32], b: &mut [i32]) {
    for i in 0..a.len() {
        unsafe {
            *a.get_unchecked_mut(i) = 1;
            *b.get_unchecked_mut(i) = 2;
        }
    }
}

#[cfg(all(feature = "fill-split", feature = "x86"))]
#[inline(never)]
fn fill(a: &mut [i32], b: &mut [i32]) {
    for x in a.iter_mut() { *x = 1; }
    for x in b.iter_mut() { *x = 2; }
}

#[cfg(feature = "x86")]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn main_core(mut a: Vec<i32>, mut b: Vec<i32>) {
    fill(black_box(&mut a), black_box(&mut b));
}
