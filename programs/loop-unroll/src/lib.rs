#[cfg(all(not(feature = "x86"), feature = "loop-unroll"))]
pub fn unroll(a: &[i32; 256]) -> i32 {
    let mut sum: i32;
    unsafe {
        std::arch::asm!(
            "li   {sum}, 0",
            "li   {i},   0",
            "2:",
            "slli t0, {i}, 2",
            "add  t1, {array}, t0", // t1 has address of a[i]
            "lw   t0, 0(t1)",
            "add  {sum}, {sum}, t0",
            "lw   t0, 4(t1)",
            "add  {sum}, {sum}, t0",
            "lw   t0, 8(t1)",
            "add  {sum}, {sum}, t0",
            "lw   t0, 12(t1)",
            "add  {sum}, {sum}, t0",
            "addi {i}, {i}, 4",
            "blt  {i}, {limit}, 2b", // if i < 256, jump back to label 2
            array = in(reg) a.as_ptr(),
            sum   = out(reg) sum,
            i     = out(reg) _,
            limit = in(reg) 256,
            out("t0") _,
            out("t1") _,
            options(nostack)
        );
    }
    sum
}

#[cfg(all(not(feature = "x86"), not(feature = "loop-unroll")))]
pub fn unroll(a: &[i32; 256]) -> i32 {
    let mut sum: i32;
    unsafe {
        std::arch::asm!(
            "li   {sum}, 0",
            "li   {i},   0",
            "2:",
            "slli t0, {i}, 2",
            "add  t1, {array}, t0", // t1 has address of a[i]
            "lw   t0, 0(t1)",
            "add  {sum}, {sum}, t0",
            "addi {i}, {i}, 1",
            "blt  {i}, {limit}, 2b", // if i < 256, jump back to label 2
            array = in(reg) a.as_ptr(),
            sum   = out(reg) sum,
            i     = out(reg) _,
            limit = in(reg) 256,
            out("t0") _,
            out("t1") _,
            options(nostack)
        );
    }
    sum
}

#[cfg(all(feature = "x86", feature = "loop-unroll"))]
pub fn unroll(a: &[i32; 256]) -> i32 {
    let mut sum: i32;
    unsafe {
        std::arch::asm!(
            "xor {sum:e}, {sum:e}",
            "mov {i}, 0",
            "2:",
            "add {sum:e}, [{array} + {i} * 4]",
            "add {sum:e}, [{array} + {i} * 4 + 4]",
            "add {sum:e}, [{array} + {i} * 4 + 8]",
            "add {sum:e}, [{array} + {i} * 4 + 12]",
            "add {i}, 4",
            "cmp {i}, 256",
            "jl 2b",
            array = in(reg) a.as_ptr(),
            sum   = out(reg) sum,
            i     = out(reg) _,
            options(nostack)
        );
    }
    sum
}

#[cfg(all(feature = "x86", not(feature = "loop-unroll")))]
pub fn unroll(a: &[i32; 256]) -> i32 {
    let mut sum: i32;
    unsafe {
        std::arch::asm!(
            "xor {sum:e}, {sum:e}",
            "mov {i}, 0",
            "2:",
            "add {sum:e}, [{array} + {i} * 4]",
            "inc {i}",
            "cmp {i}, 256",
            "jl 2b",
            array = in(reg) a.as_ptr(),
            sum = out(reg) sum,
            i = out(reg) _,
            options(nostack)
        );
    }
    sum
}

#[no_mangle]
#[cfg(feature = "x86")]
pub extern "C" fn main_core(reps: usize) -> () {
    let data: [i32; 256] = [
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200,
        210, 220, 230, 240, 250, 260, 270, 280, 290, 300, 310, 320, 10, 20, 30, 40, 50, 60, 70, 80,
        90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250, 260,
        270, 280, 290, 300, 310, 320, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140,
        150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250, 260, 270, 280, 290, 300, 310, 320,
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200,
        210, 220, 230, 240, 250, 260, 270, 280, 290, 300, 310, 320, 10, 20, 30, 40, 50, 60, 70, 80,
        90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250, 260,
        270, 280, 290, 300, 310, 320, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140,
        150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250, 260, 270, 280, 290, 300, 310, 320,
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200,
        210, 220, 230, 240, 250, 260, 270, 280, 290, 300, 310, 320, 10, 20, 30, 40, 50, 60, 70, 80,
        90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250, 260,
        270, 280, 290, 300, 310, 320,
    ];
    for _ in 0..reps {
        let res = unroll(&data);
        std::hint::black_box(res);
    }
}
