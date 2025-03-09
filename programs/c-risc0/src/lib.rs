include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn add_from_c(a: i32, b: i32) -> i32 {
    unsafe {
        c_add(a, b)
    }
}
