#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

fn factorial(n: u128) -> u128 {
    factorial_tail(n, 1)
}

fn factorial_tail(n: u128, acc: u128) -> u128 {
    if n == 0 {
        acc
    } else {
        factorial_tail(n - 1, n * acc)
    }
}

pub fn sum_to(n: u128) -> u128 {
    fn sum(n: u128, acc: u128) -> u128 {
        if n == 0 {
            acc
        } else {
            sum(n - 1, acc + n)
        }
    }
    sum(n, 0)
}

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
        let fac = factorial(n);
        let sum = sum_to(n);
        if i == r - 1 {
            println!("Factorial of {n} is: {fac}");
            println!("Sum of 1..{n} is: {sum}");
        }
    }
}
