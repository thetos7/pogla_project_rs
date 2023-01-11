use std::{cell::RefCell, collections::HashMap, ffi::CStr, rc::Rc};

use gl::types::{GLenum, GLint, GLuint};

use crate::{gl_check, gl_utils::cstring_with_null_bytes, program::shader::ShaderCompileError};

use self::{
    shader::{Shader, ShaderHandle, ShaderType},
    uniform::Uniform,
};

pub mod attribute;
pub mod shader;
pub mod uniform;

pub type ProgramIdType = GLuint;
pub type ProgramSharedPointer = Rc<RefCell<Program>>;
pub type UniformEntryType = Rc<RefCell<Uniform>>;

#[derive(Debug)]
pub struct Program {
    id: ProgramIdType,
    _shaders: Vec<ShaderHandle>, // program needs to hold onto shader handles if needed and fro simplified cleanup, maybe unnecessary
    shader_flags: u8,
    name: String,
    uniforms: HashMap<String, UniformEntryType>,
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

    pub fn has_geometry(&self) -> bool {
        self.shader_flags & ShaderType::Geometry.mask() != 0
    }

    pub fn uniform(&self, name: impl Into<String>) -> Option<&UniformEntryType> {
        self.uniforms.get(&name.into())
    }

    pub fn id(&self) -> ProgramIdType {
        self.id
    }

    pub fn name(&self) -> &String {
        &self.name
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

    fn build_uniform_map(program: &ProgramSharedPointer) {
        let mut prog = program.borrow_mut();
        let mut max_name_length: GLint = 0;
        unsafe {
            gl::GetProgramiv(prog.id, gl::ACTIVE_UNIFORM_MAX_LENGTH, &mut max_name_length);
            gl_check!();
        }
        let mut uniform_count: GLint = 0;
        unsafe {
            gl::GetProgramiv(prog.id, gl::ACTIVE_UNIFORMS, &mut uniform_count);
            gl_check!();
        }
        let uniform_count = uniform_count;

        let mut name: Vec<u8> = vec![0; max_name_length as usize];

        for i in 0..uniform_count as u32 {
            let mut uniform_type: GLenum = 0;
            let mut size: GLint = 0;
            let mut length = 0;

            unsafe {
                gl::GetActiveUniform(
                    prog.id,
                    i,
                    max_name_length + 1,
                    &mut length,
                    &mut size,
                    &mut uniform_type,
                    name.as_mut_ptr() as _,
                );
                gl_check!();
            }

            let loc = unsafe {
                let loc = gl::GetUniformLocation(prog.id, name.as_ptr() as _);
                gl_check!();
                loc
            };

            let name = unsafe {
                CStr::from_ptr(name.as_ptr() as _)
                    .to_owned()
                    .to_string_lossy()
                    .into_owned()
            };

            prog.uniforms.insert(
                name.clone(),
                Rc::new(RefCell::new(Uniform::new(
                    name,
                    loc,
                    uniform_type,
                    size,
                    Rc::downgrade(&program),
                ))),
            );
        }
    }

    #[must_use]
    pub fn build<'a>(self) -> Result<ProgramSharedPointer, ProgramBuildError> {
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
                let program_log = cstring_with_null_bytes(log_size as usize + 1);
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

            let prog = Rc::new(RefCell::new(Program {
                id: program_id,
                shader_flags,
                _shaders: compiled_shaders,
                name: self.name,
                uniforms: Default::default(),
            }));
            Self::build_uniform_map(&prog);
            Ok(prog)
        }
    }
}
