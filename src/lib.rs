#![no_std]
extern crate alloc;
use cstr_core::*;
use crate::alloc::borrow::ToOwned;
use alloc::string::String;

pub type CString = i32;

pub fn cstr(s: &str) -> CString {
    cstr_core::CString::new(s).unwrap().into_raw() as i32
}

pub fn cstr_to_string(c: CString) -> String {
    let s: &CStr = unsafe { CStr::from_ptr(c as *const i8) };
    s.to_str().unwrap().to_owned()
}
