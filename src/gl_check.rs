pub(crate) fn check_gl_error(file: &str, line: u32) -> bool {
    let error_code = unsafe { gl::GetError() };
    if error_code != gl::NO_ERROR {
        println!(
            "[OpenGL Error] {file} at line {line}: got code `0x{code:x}` ({code})",
            code = error_code
        );
        return true;
    }
    false
}

#[macro_export]
macro_rules! gl_check {
    () => {
        crate::gl_check::check_gl_error(file!(), line!())
    };
}
