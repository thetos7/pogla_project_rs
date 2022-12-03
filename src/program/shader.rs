use std::{
    ffi::CString,
    fs, io,
    ptr::{null, null_mut},
};

use crate::{gl_check, gl_utils::new_log_buffer};

#[derive(Clone, Copy)]
#[allow(unused)]
pub enum ShaderType {
    Vertex,
    TesselationControl,
    TesselationEvaluation,
    Geometry,
    Fragment,
    Compute,
}

impl ShaderType {
    pub const fn gl_constant(self) -> gl::types::GLenum {
        match self {
            Self::Vertex => gl::VERTEX_SHADER,
            Self::TesselationControl => gl::TESS_CONTROL_SHADER,
            Self::TesselationEvaluation => gl::TESS_EVALUATION_SHADER,
            Self::Geometry => gl::GEOMETRY_SHADER,
            Self::Fragment => gl::FRAGMENT_SHADER,
            Self::Compute => gl::COMPUTE_SHADER,
        }
    }

    pub const fn mask(self) -> u8 {
        match self {
            Self::Vertex => 1 << 0,
            Self::TesselationControl => 1 << 1,
            Self::TesselationEvaluation => 1 << 2,
            Self::Geometry => 1 << 3,
            Self::Fragment => 1 << 4,
            Self::Compute => 1 << 5,
        }
    }
}

impl ToString for ShaderType {
    fn to_string(&self) -> String {
        match *self {
            Self::Vertex => "vertex",
            Self::TesselationControl => "tess_control",
            Self::TesselationEvaluation => "tess_evaluation",
            Self::Geometry => "geometry",
            Self::Fragment => "fragment",
            Self::Compute => "compute",
        }
        .into()
    }
}

pub struct Shader {
    shader_type: ShaderType,
    sources: Vec<String>,
}

pub struct ShaderHandle {
    pub shader_type: ShaderType,
    pub id: gl::types::GLuint,
}

pub enum ShaderCompileError {
    CompilationError(String),
}

impl Shader {
    pub fn shader_type(&self) -> ShaderType {
        self.shader_type
    }

    pub fn new(shader_type: ShaderType) -> Self {
        Shader {
            shader_type,
            sources: vec![],
        }
    }
    pub fn load(mut self, path: &str) -> io::Result<Self> {
        self.sources.push(fs::read_to_string(path)?);
        Ok(self)
    }

    pub fn source(mut self, source: &str) -> Self {
        self.sources.push(source.into());
        self
    }

    pub fn compile(self) -> Result<ShaderHandle, ShaderCompileError> {
        unsafe {
            let shader_id = gl::CreateShader(self.shader_type.gl_constant());
            gl_check!();

            let sources_cstr = self
                .sources
                .iter()
                .map(|s| CString::new(s.as_bytes()).unwrap())
                .collect::<Vec<_>>(); // necessary to keep sources allocated until the end
            let sources = sources_cstr.iter().map(|s| s.as_ptr()).collect::<Vec<_>>();
            gl::ShaderSource(
                shader_id,
                self.sources.len() as i32,
                sources.as_ptr(),
                null(),
            );
            gl_check!();

            gl::CompileShader(shader_id);
            gl_check!();

            let mut compile_status: gl::types::GLint = gl::TRUE as _;
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut compile_status);
            if compile_status != gl::TRUE as _ {
                let mut log_size: gl::types::GLint = 0;
                gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut log_size);
                gl_check!();
                let shader_log: CString = new_log_buffer(log_size as usize + 1);
                gl::GetShaderInfoLog(shader_id, log_size, null_mut(), shader_log.as_ptr() as _);
                gl_check!();
                gl::DeleteShader(shader_id);
                gl_check!();

                return Err(ShaderCompileError::CompilationError(
                    shader_log.to_string_lossy().into_owned(),
                ));
            }

            Ok(ShaderHandle {
                shader_type: self.shader_type,
                id: shader_id,
            })
        }
    }
}

impl Drop for ShaderHandle {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}
