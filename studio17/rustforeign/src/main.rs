use std::ffi::CString;
use std::{env::args, os::raw::c_int};

extern crate libc;

extern "C" {
    pub fn printf(format: *const libc::c_char, ...) -> libc::c_int;
}

fn main() {
    let arg_count: c_int = args().count() as c_int;
    let format = CString::new("Program argument count: %i\n").unwrap();

    unsafe {
        printf(format.as_ptr(), arg_count);
    }

    let per_arg_format = CString::new("%s\n").unwrap();

    for arg in args() {
        let arg_cstr = CString::new(arg).unwrap();

        unsafe {
            printf(per_arg_format.as_ptr(), arg_cstr.as_ptr());
        }
    }
}
