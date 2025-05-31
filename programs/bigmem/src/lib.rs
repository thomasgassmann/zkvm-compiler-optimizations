#[macro_export]
macro_rules! bigmem_bench {
    ($value:expr) => {{
        let array = [$value; 128000];
        core::hint::black_box(array);
        let result = array[16000];
        result
    }};
}

#[cfg(feature = "x86")]
#[no_mangle]
pub extern "C" fn main_core(value: u32) {
    let result = bigmem_bench!(value);
    core::hint::black_box(result);
}
