// https://github.com/RustAudio/rust-jack/blob/6133ac4d3654668f461a036fef8e3d9655b6c372/src/jack_utils.rs

use jack_sys as j;
use std::ffi;

pub unsafe fn collect_c_strings(ptr: *const *const libc::c_char) -> Vec<String> {
    if ptr.is_null() {
        return Vec::new();
    };
    let len = {
        let mut len = 0;
        while !(*ptr.offset(len)).is_null() {
            len += 1;
        }
        len
    };
    let mut strs = Vec::with_capacity(len as usize);
    for i in 0..len {
        let cstr_ptr = *ptr.offset(i);
        let s = ffi::CStr::from_ptr(cstr_ptr).to_string_lossy().into_owned();
        strs.push(s);
    }
    j::jack_free(ptr as *mut ::libc::c_void);
    strs
}
