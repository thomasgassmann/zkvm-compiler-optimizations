#![no_main]

sp1_zkvm::entrypoint!(main);

use c_sp1::add_from_c;

fn main() {
    println!("C: 1 + 2 = {}", add_from_c(1, 2));
}
