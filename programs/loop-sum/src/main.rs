#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

#[inline(never)]
fn sum_array(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in arr.iter() {
        sum += val;
    }
    sum
}

pub fn main() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = sum_array(&data);
    println!("result: {}", result);
}
