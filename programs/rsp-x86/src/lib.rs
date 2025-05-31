use rsp_client_executor::{io::ClientExecutorInput, ClientExecutor, EthereumVariant};
use rayon::ThreadPoolBuilder;

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub fn main_core(input: &Vec<u8>) {
    // Read the input.
    let input = bincode::deserialize::<ClientExecutorInput>(&input).unwrap();

    let single = ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .unwrap();

    // Execute the block.
    single.install(|| {
        let executor = ClientExecutor;
        let header = executor
            .execute::<EthereumVariant>(input)
            .expect("failed to execute client");
        let block_hash = header.hash_slow();

        core::hint::black_box(&block_hash);
    });
}
