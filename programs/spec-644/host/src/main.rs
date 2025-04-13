#![no_main]
#![allow(unused)]
#![feature(c_variadic)]

use c_platform::include_platform;

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

include_platform!();

#[link(name = "spec-644", kind = "static")]
extern "C" {
    fn cmain() -> ();
}

fn main() {
    unsafe {
        cmain();
    }
}
