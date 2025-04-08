#![feature(c_variadic)]

pub use printf_compat;

#[macro_export]
macro_rules! include_platform {
    () => {
        use std::os::raw::{c_char, c_int};
        use $crate::printf_compat::{format, argument::Argument, argument::Specifier};
        use core::str;
        use core::fmt;
        use std::alloc::{alloc, dealloc, Layout};
        use std::mem;
        use std::ptr;

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

        #[unsafe(no_mangle)]
        pub extern "C" fn malloc(size: i32) -> *mut u8 {
            unsafe {
                let header_size = mem::size_of::<usize>();
                let user_size = size as usize;
                let total_size = header_size.checked_add(user_size)
                    .expect("Size overflow");

                let layout = Layout::from_size_align(total_size, mem::align_of::<usize>())
                    .expect("Invalid layout");

                let raw_ptr = alloc(layout);
                if raw_ptr.is_null() {
                    panic!("malloc failed");
                }

                (raw_ptr as *mut usize).write(total_size);
                // Return a pointer to memory immediately after the header
                raw_ptr.add(header_size)
            }
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn free(ptr: *mut u8) {
            unsafe {
                if ptr.is_null() {
                    return;
                }

                let header_size = mem::size_of::<usize>();
                let orig_ptr = ptr.sub(header_size);
                let total_size = (orig_ptr as *mut usize).read();
                let layout = Layout::from_size_align(total_size, mem::align_of::<usize>())
                    .expect("Invalid layout");
                dealloc(orig_ptr, layout);
            }
        }
    };
}
