use ansi_term::Color;

fn stringify_gl_error_code(code: u32) -> String {
    match code {
        gl::INVALID_ENUM => "GL_INVALID_ENUM",
        gl::INVALID_VALUE => "GL_INVALID_VALUE",
        gl::INVALID_OPERATION => "GL_INVALID_OPERATION",
        gl::STACK_OVERFLOW => "GL_STACK_OVERFLOW",
        gl::STACK_UNDERFLOW => "GL_STACK_UNDERFLOW",
        gl::OUT_OF_MEMORY => "GL_OUT_OF_MEMORY",
        gl::INVALID_FRAMEBUFFER_OPERATION => "GL_INVALID_FRAMEBUFFER_OPERATION",
        gl::CONTEXT_LOST => "GL_CONTEXT_LOST",
        _ => "<unknown-error>",
    }
    .into()
}

pub(crate) fn check_gl_error(file: &str, line: u32, sub_expression: Option<u32>) -> bool {
    let error_code = unsafe { gl::GetError() };
    if error_code != gl::NO_ERROR {
        let header = Color::Red.paint("[OpenGL Error]");

        let line_detail = match sub_expression {
            None => format!("at line {}", Color::Cyan.paint(line.to_string())),
            Some(i) => format!(
                "at line {line} sub-expression {i}",
                line = Color::Cyan.paint(line.to_string()),
                i = Color::Cyan.paint(i.to_string())
            ),
        };

        eprintln!(
            "{header} {file} {line_detail}: got code `0x{code:x}` ({code}): `{code_str}`",
            file = Color::Green.paint(file),
            code = error_code,
            code_str = Color::Yellow.paint(stringify_gl_error_code(error_code))
        );

        return true;
    }
    false
}

/// Macro to check a single time if an openGL error occurred.
///
/// The expanded expression is a boolean which is true if an error occurred.
#[macro_export(local_inner_macro)]
macro_rules! gl_check {
    () => {
        gl_check!(file!(), line!())
    };
    ($file:expr, $line:expr) => {
        $crate::gl_check::check_gl_error($file, $line, None)
    };
}

/// Macro that adds a call to the custom openGL error checking function after each statement.
///
/// The expanded expression resolves to a boolean, which is true if *any* error occurred.
#[macro_export(local_inner_macro)]
macro_rules! gl_checked {
    (@step $_idx:expr, []) => {false};
    (@step $idx:expr, [$head:stmt; $($tail:stmt; )*]) => {{
        $head
        let error = $crate::gl_check::check_gl_error(file!(), line!(), Some($idx));
        gl_checked!(@step $idx + 1, [$($tail;)*]) || error
    }};
    {$( $l:stmt; )+} => {
        {
            gl_checked!(@step 1, [$($l;)*])
        }
    };
}
