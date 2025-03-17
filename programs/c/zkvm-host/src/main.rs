#![no_main]
#![allow(unused)]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

use zkvmlib;

#[link(name = "zkvmc", kind = "static")]
extern "C" {
    fn cmain() -> ();
}

fn main() {
    unsafe {
        cmain();
    }
}
