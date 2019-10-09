#![no_std]
mod cstr_core;
extern crate alloc;
use crate::alloc::borrow::ToOwned;
use alloc::string::String;

pub type CString = i32;

pub fn cstr(s: &str) -> CString {
    cstr_core::CString::new(s).unwrap().into_raw() as i32
}

pub fn cstr_to_string(c: CString) -> String {
    let s: &cstr_core::CStr = unsafe { cstr_core::CStr::from_ptr(c as *const cstr_core::c_char) };
    s.to_str().unwrap().to_owned()
}
