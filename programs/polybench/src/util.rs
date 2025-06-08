use std::fmt;

pub fn consume<T: fmt::Display>(dummy: T) -> T {
    #[cfg(feature = "print-result")]
    println!("{}", &dummy);

    core::hint::black_box(&dummy);
    dummy
}
