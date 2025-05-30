pub fn factorial(n: u128) -> u128 {
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

#[cfg(feature = "x86")]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn main_core(n: u128, r: u128) {
    for i in 0..r {
        let fac = factorial(n);
        let sum = sum_to(n);
        if i == r - 1 {
            println!("Factorial of {n} is: {fac}");
            println!("Sum of 1..{n} is: {sum}");
        }
    }
}
