#[cfg(feature = "x86")]
use sha2::{Digest, Sha256};

#[macro_export]
macro_rules! sha256_hash {
    ($input:expr) => {{
        let mut hasher = Sha256::new();
        hasher.update($input);
        hasher.finalize()
    }};
}

#[cfg(feature = "x86")]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn main_core(input: Vec<u8>) {
    let result = sha256_hash!(input);
    core::hint::black_box(result);
}
