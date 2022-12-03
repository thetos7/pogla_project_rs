use std::ffi::CString;

pub fn new_log_buffer(length: usize) -> CString {
    unsafe { CString::from_vec_unchecked(vec![b'\0'; length]) }
}
