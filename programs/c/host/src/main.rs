#![no_main]
#![allow(unused)]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn printsomething(a: i32) -> () {
    println!("Hello from Rust! {}", a);
}

#[link(name = "c", kind = "static")]
extern "C" {
    fn cmain() -> ();
}

fn main() {
    unsafe {
        cmain();
    }
}
