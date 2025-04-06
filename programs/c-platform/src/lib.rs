#![feature(c_variadic)]

pub use printf_compat;

#[macro_export]
macro_rules! include_platform {
    () => {
        use std::os::raw::{c_char, c_int};
        use $crate::printf_compat::{format, argument::Argument, argument::Specifier};
        use core::str;
        use core::fmt;

        pub fn fmt_write() -> impl FnMut(Argument) -> c_int {
            move |arg: Argument| -> c_int {
                let output = match arg.specifier {
                    Specifier::Percent => "%".to_string(),
                    Specifier::Bytes(data) => {
                        match str::from_utf8(data) {
                            Ok(s) => s.to_string(),
                            Err(_) => "<invalid utf8>".to_string(),
                        }
                    }
                    Specifier::String(s) => {
                        match s.to_str() {
                            Ok(st) => st.to_string(),
                            Err(_) => "<invalid CStr>".to_string(),
                        }
                    }
                    Specifier::Uint(num) => format!("{}", num),
                    Specifier::Int(num) => format!("{}", num),
                    Specifier::Char(c) => format!("{}", c as char),
                    _ => {
                        "<unsupported specifier>".to_string()
                    }
                };
        
                print!("{}", output);
                output.len() as c_int
            }
        }
        
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn printf(str: *const c_char, mut args: ...) -> c_int {
            unsafe {
                let bytes_written = format(
                    str,
                    args.as_va_list(),
                    fmt_write(),
                );
                bytes_written
            }
        }
    };
}
