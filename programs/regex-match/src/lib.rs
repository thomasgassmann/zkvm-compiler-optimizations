#[cfg(feature = "x86")]
use regex::Regex;

#[macro_export]
macro_rules! regex_match_and_commit {
    ($t:expr, $r:expr) => {
        let re = Regex::new(&$r.as_str()).unwrap();
        for str in re
            .find_iter(&$t.as_str())
            .map(|m| m.as_str())
            .collect::<Vec<&str>>()
        {
            #[cfg(any(feature = "risc0", feature = "sp1"))]
            println!("{}", str);
            #[cfg(feature = "sp1")]
            sp1_zkvm::io::commit(&str);
            #[cfg(feature = "risc0")]
            risc0_zkvm::guest::env::commit(&str);
            #[cfg(feature = "x86")]
            core::hint::black_box(str);
        }
    };
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
#[cfg(feature = "x86")]
pub extern "C" fn main_core(r: String, t: String) -> () {
    regex_match_and_commit!(t, r);
}

