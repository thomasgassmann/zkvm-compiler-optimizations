#![no_main]

use regex::Regex;

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

fn main() {
    #[cfg(feature = "risc0")]
    let r: String = risc0_zkvm::guest::env::read();
    #[cfg(feature = "sp1")]
    let r: String = sp1_zkvm::io::read();
    #[cfg(feature = "risc0")]
    let t: String = risc0_zkvm::guest::env::read();
    #[cfg(feature = "sp1")]
    let t: String = sp1_zkvm::io::read();

    let re = Regex::new(&r.as_str()).unwrap();
    for str in re.find_iter(&t.as_str()).map(|m| m.as_str()).collect::<Vec<&str>>() {
        println!("{}", str);
        #[cfg(feature = "sp1")]
        sp1_zkvm::io::commit(&str);
        #[cfg(feature = "risc0")]
        risc0_zkvm::guest::env::commit(&str);
    }
}
