#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

fn main() {
    #[cfg(feature = "risc0")]
    let n: u128 = risc0_zkvm::guest::env::read();
    #[cfg(feature = "sp1")]
    let n: u128 = sp1_zkvm::io::read();

    #[cfg(feature = "risc0")]
    let r: u128 = risc0_zkvm::guest::env::read();
    #[cfg(feature = "sp1")]
    let r: u128 = sp1_zkvm::io::read();

    for i in 0..r {
        let fac = tailcall::factorial(n);
        let sum = tailcall::sum_to(n);
        if i == r - 1 {
            println!("Factorial of {n} is: {fac}");
            println!("Sum of 1..{n} is: {sum}");
        }
    }
}
