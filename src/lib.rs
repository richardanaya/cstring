pub fn cstr(s: &str) -> i32 {
    std::ffi::CString::new(s).unwrap().into_raw() as i32
}
