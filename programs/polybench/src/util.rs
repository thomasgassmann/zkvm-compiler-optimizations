use std::fmt;

pub fn consume<T: fmt::Display>(dummy: T) -> T {
    #[cfg(feature = "print-result")]
    println!("{}", &dummy);

    unsafe {
        // Taken from bencher crate:
        // https://docs.rs/bencher/0.1.5/src/bencher/lib.rs.html#590-596
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}
