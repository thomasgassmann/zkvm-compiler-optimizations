#![no_main]
#![allow(unused)]
#![feature(c_variadic)]

use std::ffi::{CString, CStr};

use c_platform::include_platform;

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

include_platform!();

#[link(name = "spec-605", kind = "static")]
extern "C" {
    fn cmain() -> ();
}

static INPUT: &str = include_str!("inp_small.in");

#[no_mangle]
pub unsafe fn get_input() -> *const u8 {
    let cstr = CString::new(INPUT).unwrap();
    cstr.into_raw() as *const u8
}

fn main() {
    unsafe {
        cmain();
    }
}
