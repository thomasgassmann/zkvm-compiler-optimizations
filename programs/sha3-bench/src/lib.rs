#[cfg(feature = "x86")]
use sha3::{Digest, Keccak256};

#[macro_export]
macro_rules! sha3_hash {
    ($input:expr) => {{
        let mut hasher = Keccak256::new();
        hasher.update($input);
        hasher.finalize()
    }};
}

#[cfg(feature = "x86")]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn main_core(input: Vec<u8>) {
    let result = sha3_hash!(input);
    core::hint::black_box(result);
}
