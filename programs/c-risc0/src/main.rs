// #![no_main]

// risc0_zkvm::guest::entry!(main);

use c_risc0::add_from_c;

fn main() {
    println!("C: 1 + 2 = {}", add_from_c(1, 2));
}
