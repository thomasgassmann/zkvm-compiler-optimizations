// https://github.com/nanpuyue/sha256

#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

fn main() {
    for i in 0..10 {
        let mut c = sha256::Sha256::default();
        c.update(sha256::generate_string(i).as_bytes());
        println!("{}", sha256::to_hex(&c.finish()).as_str());
    }
}
