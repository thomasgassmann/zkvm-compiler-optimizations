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
        pub extern "C" fn calloc(count: i32, size: i32) -> *mut u8 {
            unsafe {
                let total_size = (count as usize).checked_mul(size as usize)
                    .expect("Size overflow");
                let ptr = malloc(total_size as i32);
                ptr::write_bytes(ptr, 0, total_size);
                ptr
            }
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn realloc(ptr: *mut u8, size: i32) -> *mut u8 {
            unsafe {
                if ptr.is_null() {
                    return malloc(size);
                }

                if size == 0 {
                    free(ptr);
                    return ptr::null_mut();
                }

                let header_size = mem::size_of::<usize>();

                let orig_ptr = ptr.sub(header_size);

                let old_total_size = (orig_ptr as *mut usize).read();
                let old_user_size = old_total_size - header_size;

                let new_user_size = size as usize;
                let new_total_size = header_size.checked_add(new_user_size)
                    .expect("Size overflow");

                let new_layout = Layout::from_size_align(new_total_size, mem::align_of::<usize>())
                    .expect("Invalid layout");

                let new_raw_ptr = alloc(new_layout);
                if new_raw_ptr.is_null() {
                    panic!("realloc failed: allocation returned null");
                }

                (new_raw_ptr as *mut usize).write(new_total_size);

                let new_user_ptr = new_raw_ptr.add(header_size);

                let copy_size = if old_user_size < new_user_size { old_user_size } else { new_user_size };
                ptr::copy_nonoverlapping(ptr, new_user_ptr, copy_size);

                let old_layout = Layout::from_size_align(old_total_size, mem::align_of::<usize>())
                    .expect("Invalid layout");
                dealloc(orig_ptr, old_layout);

                new_user_ptr
            }
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn strcpy(dest: *mut u8, src: *const u8) -> *mut u8 {
            let mut i = 0;
            loop {
                let byte = *src.add(i);
                *dest.add(i) = byte;
                if byte == 0 {
                    break;
                }
                i += 1;
            }
            dest
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn atoi(s: *const u8) -> i32 {
            if s.is_null() {
                return 0;
            }

            let mut ptr = s;
            let mut result: i32 = 0;
            let mut sign: i32 = 1;

            while *ptr == b' ' {
                ptr = ptr.add(1);
            }

            if *ptr == b'-' {
                sign = -1;
                ptr = ptr.add(1);
            } else if *ptr == b'+' {
                ptr = ptr.add(1);
            }

            while *ptr >= b'0' && *ptr <= b'9' {
                let digit = (*ptr - b'0') as i32;

                if let Some(new_result) = result.checked_mul(10).and_then(|r| r.checked_add(digit)) {
                    result = new_result;
                } else {
                    return if sign == 1 { i32::MAX } else { i32::MIN };
                }

                ptr = ptr.add(1);
            }

            result * sign
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

        #[unsafe(no_mangle)]
        #[inline(always)]
        pub extern "C" fn exit(code: i32) {
            unsafe {
                std::process::exit(code);
            }
        }

        #[unsafe(no_mangle)]
        #[inline(always)]
        pub extern "C" fn read_int() -> i32 {
            #[cfg(feature = "risc0")]
            let n: i32 = risc0_zkvm::guest::env::read();
            #[cfg(feature = "sp1")]
            let n: i32 = sp1_zkvm::io::read();
            n
        }
    };
}
