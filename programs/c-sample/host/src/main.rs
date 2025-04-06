#![no_main]
#![allow(unused)]
#![feature(c_variadic)]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

use std::os::raw::{c_char, c_int};
use printf_compat::{format, output};

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn printsomething(a: i32) -> () {
    println!("Hello from Rust! {}", a);
}

#[no_mangle]
pub unsafe extern "C" fn printf(str: *const c_char, mut args: ...) -> c_int {
    let mut s = String::new();
    let bytes_written = format(str, args.as_va_list(), output::fmt_write(&mut s));
    print!("{}", s);
    bytes_written
}

#[link(name = "c-sample", kind = "static")]
extern "C" {
    fn cmain() -> ();
}

fn main() {
    unsafe {
        cmain();
    }
}
