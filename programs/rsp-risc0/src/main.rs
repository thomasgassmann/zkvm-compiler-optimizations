#![no_main]

use rsp_client_executor::{io::ClientExecutorInput, ClientExecutor, EthereumVariant};

risc0_zkvm::guest::entry!(main);

fn main() {
    let vec: Vec<u8> = risc0_zkvm::guest::env::read();

    let input = bincode::deserialize::<ClientExecutorInput>(&vec).unwrap();

    let executor = ClientExecutor;
    let header = executor.execute::<EthereumVariant>(input).expect("failed to execute client");
    let block_hash = header.hash_slow();

    risc0_zkvm::guest::env::commit(&block_hash);
}
