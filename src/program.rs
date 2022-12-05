use crate::{gl_check, gl_utils::new_log_buffer, program::shader::ShaderCompileError};

use self::shader::{Shader, ShaderHandle, ShaderType};

pub mod shader;
pub mod uniform;
pub mod attribute;

pub struct Program {
    id: gl::types::GLuint,
    _shaders: Vec<ShaderHandle>, // program needs to hold onto shader handles if needed and fro simplified cleanup, maybe unnecessary
    shader_flags: u8,
    name: String,
}

impl Program {
    pub fn builder(name: &str) -> ProgramBuilder {
        ProgramBuilder {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn unbind() {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
            gl_check!();
        }
    }

    pub fn is_compute(&self) -> bool {
        self.shader_flags & ShaderType::Compute.mask() != 0
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        // unbind program if current
        let mut current_program = 0i32;
        unsafe {
            gl::GetIntegerv(gl::CURRENT_PROGRAM, &mut current_program);
            gl_check!();
        }
        if current_program == self.id as _ {
            Program::unbind();
            log::warn!(
                "Unbinding current program number {id} (`{name}`) because of dropping",
                id = self.id,
                name = self.name
            )
        }

        // delete program
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

#[derive(Default)]
pub struct ProgramBuilder {
    name: String,
    shaders: Vec<(String, Shader)>,
}

pub enum ProgramBuildError {
    CreationFailed,
    LinkFail(String),
}

impl ProgramBuildError {
    pub fn log_error(&self) {
        match self {
            Self::CreationFailed => log::error!("program creation failed"),
            Self::LinkFail(log) => log::error!("Program linking failed, link log:\n{log}"),
        }
    }
}

impl ProgramBuilder {
    pub fn add_shader(mut self, name: &str, shader: Shader) -> Self {
        self.shaders.push((name.into(), shader));
        self
    }

    #[must_use]
    pub fn build(self) -> Result<Program, ProgramBuildError> {
        unsafe {
            let program_id = gl::CreateProgram();
            gl_check!();

            if program_id == 0 {
                return Err(ProgramBuildError::CreationFailed);
            }

            let mut shader_flags = 0u8;

            let compiled_shaders: Vec<_> = self
                .shaders
                .into_iter()
                .filter_map(|(name, shader)| {
                    let mask = shader.shader_type().mask();
                    if shader_flags & mask != 0 {
                        log::warn!(
                            "Program already has shader of type: `{}`",
                            shader.shader_type().to_string()
                        )
                    }
                    shader_flags |= mask;
                    let compilation_result = shader.compile();
                    match compilation_result {
                        Err(error) => {
                            match error {
                                ShaderCompileError::CompilationError(log) => {
                                    log::warn!(
                                        "[{prog_name}]<{name}> Shader compilation error.\n---- LOG ----\n{log}",
                                        prog_name = self.name
                                    )
                                }
                            }
                            None
                        }
                        Ok(handle) => Some( handle),
                    }
                })
                .collect();

            for handle in &compiled_shaders {
                gl::AttachShader(program_id, handle.id);
            }
            let mut link_status = gl::TRUE as _;
            gl::LinkProgram(program_id);
            gl_check!();
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut link_status);
            gl_check!();

            if link_status != gl::TRUE as _ {
                let mut log_size = 0;
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut log_size);
                gl_check!();
                let program_log = new_log_buffer(log_size as usize + 1);
                gl::GetProgramInfoLog(
                    program_id,
                    log_size,
                    &mut log_size,
                    program_log.as_ptr() as _,
                );
                gl_check!();

                gl::DeleteProgram(program_id);
                return Err(ProgramBuildError::LinkFail(
                    program_log.to_string_lossy().into_owned(),
                ));
            }

            Ok(Program {
                id: program_id,
                shader_flags,
                _shaders: compiled_shaders,
                name: self.name,
            })
        }
    }
}
