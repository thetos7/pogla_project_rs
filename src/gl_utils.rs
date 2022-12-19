use std::ffi::CString;

pub fn cstring_with_null_bytes(length: usize) -> CString {
    unsafe { CString::from_vec_unchecked(vec![b'\0'; length]) }
}
