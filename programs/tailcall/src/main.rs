#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

fn factorial(n: u64) -> u64 {
    factorial_tail(n, 1)
}

fn factorial_tail(n: u64, acc: u64) -> u64 {
    if n == 0 {
        acc
    } else {
        factorial_tail(n - 1, n * acc)
    }
}

pub fn sum_to(n: i32) -> i32 {
    fn sum(n: i32, acc: i32) -> i32 {
        if n == 0 {
            acc
        } else {
            sum(n - 1, acc + n)
        }
    }
    sum(n, 0)
}

fn main() {
    let result = factorial(8);
    println!("Factorial of 5 is: {}", result);

    let result = sum_to(20);
    println!("Sum of 1..20 is: {}", result);
}
