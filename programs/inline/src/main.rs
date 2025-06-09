#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    #[cfg(feature = "risc0")]
    let n: u32 = risc0_zkvm::guest::env::read();
    #[cfg(feature = "sp1")]
    let n: u32 = sp1_zkvm::io::read();
    for i in 0..n {
        #[cfg(feature = "inline")]
        let res = inline::work_inlined(i as u64);
        #[cfg(not(feature = "inline"))]
        let res = inline::work_non_inlined(i as u64);
        core::hint::black_box(res);
    }
}
