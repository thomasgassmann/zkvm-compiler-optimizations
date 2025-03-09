#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

extern crate libc;

#[link(name = "hello", kind = "static")]
extern "C" {
    fn hello() -> i32;
}

fn main() {
    unsafe {
        println!("{}", hello());
    }
}
