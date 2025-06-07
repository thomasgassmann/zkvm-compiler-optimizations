#![no_main]

use loopunroll::matmul;

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    #[cfg(feature = "risc0")]
    let reps: usize = risc0_zkvm::guest::env::read();
    #[cfg(feature = "sp1")]
    let reps: usize = sp1_zkvm::io::read();
    let matrix = [
        [1.0, 2.0, 3.0, 4.0, 5.0],
        [2.0, 3.0, 4.0, 5.0, 6.0],
        [3.0, 4.0, 5.0, 6.0, 7.0],
        [4.0, 5.0, 6.0, 7.0, 8.0],
        [5.0, 6.0, 7.0, 8.0, 9.0],
    ];
    let vector = [1.0, 2.0, 4.0, 8.0, 16.0];
    for _ in 0..reps {
        let res = matmul(&matrix, &vector);
        std::hint::black_box(res);
    }
}
