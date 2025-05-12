#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

#[inline(never)]
fn sum_array(arr: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for &val in arr.iter() {
        sum += val;
    }
    sum
}

pub fn main() {
    #[cfg(feature = "risc0")]
    let data: Vec<i32> = risc0_zkvm::guest::env::read();
    #[cfg(feature = "sp1")]
    let data: Vec<i32> = sp1_zkvm::io::read();

    let result = sum_array(&data);
    println!("result: {}", result);
}
