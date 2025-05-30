#[cfg(feature = "x86")]
use sha2::{Digest, Sha256};

#[macro_export]
macro_rules! sha256_hash {
    ($input:expr, $num_iters:expr) => {{ 
        let mut hash = $input;
        for _ in 0..$num_iters {
            let mut hasher = Sha256::new();
            hasher.update($input);
            let res = &hasher.finalize();
            hash = Into::<[u8; 32]>::into (*res);
        }

        hash
    }};
}

#[cfg(feature = "x86")]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn main_core(input: [u8; 32], num_iters: u32) {
    let result = sha256_hash!(input, num_iters);
    core::hint::black_box(result);
}
