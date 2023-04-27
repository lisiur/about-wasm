#![allow(clippy::missing_safety_doc)]

use std::{
    ffi::{c_char, CStr, CString},
    os::raw::c_void,
};

#[no_mangle]
pub fn allocate(size: usize) -> *mut c_void {
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    std::mem::forget(buffer);

    pointer as *mut c_void
}

#[no_mangle]
pub unsafe fn deallocate(pointer: *mut c_void, size: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(pointer, 0, size);
    }
}

#[no_mangle]
pub unsafe fn hello(name: *mut c_char) -> *mut c_char {
    let name = unsafe { CStr::from_ptr(name).to_bytes().to_vec() };
    let mut output = b"Hello, ".to_vec();
    output.extend(&name);
    output.extend(&[b'!']);

    unsafe { CString::from_vec_unchecked(output) }.into_raw()
}
