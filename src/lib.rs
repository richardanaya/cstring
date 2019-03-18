use std::ffi::CStr;

pub type CString = i32;

pub fn cstr(s: &str) -> CString {
    std::ffi::CString::new(s).unwrap().into_raw() as i32
}

pub fn cstr_to_owned(c: CString) -> String {
    let s: &CStr = unsafe { CStr::from_ptr(c as *const i8) };
    s.to_str().unwrap().to_owned()
}
