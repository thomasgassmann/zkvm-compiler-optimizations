#![no_main]

use zkvmmnist::NeuralNetwork;

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

#[allow(dead_code)]
fn main() {
    #[cfg(all(not(feature = "risc0"), not(feature = "sp1")))]
    let training_data: Vec<(Vec<f64>, Vec<f64>)> = vec![];
    #[cfg(all(not(feature = "risc0"), not(feature = "sp1")))]
    let test_data: Vec<(Vec<f64>, Vec<f64>)> = vec![];

    #[cfg(feature = "risc0")]
    let training_data: Vec<(Vec<f64>, Vec<f64>)> = risc0_zkvm::guest::env::read();
    #[cfg(feature = "sp1")]
    let training_data: Vec<(Vec<f64>, Vec<f64>)> = sp1_zkvm::io::read();

    #[cfg(feature = "risc0")]
    let test_data: Vec<(Vec<f64>, Vec<f64>)> = risc0_zkvm::guest::env::read();
    #[cfg(feature = "sp1")]
    let test_data: Vec<(Vec<f64>, Vec<f64>)> = sp1_zkvm::io::read();

    println!("Training mnist");
    let mut nn = NeuralNetwork::new(49, 8, 10);
    nn.train(&training_data, &test_data, 2, 1.0);
}
