use cgmath::{Matrix4, Vector3, Vector4};

use crate::gl_check;

use super::Program;

type LocType = gl::types::GLuint;
type SizeType = gl::types::GLint;
type TypeEnum = gl::types::GLenum;

fn stringify_type(value_type: TypeEnum) -> String {
    match value_type {
        gl::FLOAT => "float".to_owned(),
        gl::FLOAT_MAT4 => "mat4".to_owned(),
        gl::FLOAT_MAT3 => "mat3".to_owned(),
        gl::FLOAT_MAT2 => "mat2".to_owned(),
        gl::FLOAT_VEC4 => "vec4".to_owned(),
        gl::FLOAT_VEC3 => "vec3".to_owned(),
        gl::FLOAT_VEC2 => "vec2".to_owned(),
        gl::INT => "int".to_owned(),
        gl::SAMPLER_2D => "sampler2D".to_owned(),
        _ => format!("<unknown type 0x{value_type:x} ({value_type})>"),
    }
}

pub struct Uniform<'a> {
    location: LocType,
    size: SizeType,
    value_type: TypeEnum,
    name: String,
    program: &'a Program,
}

impl<'a> Uniform<'a> {
    pub fn new(
        name: String,
        location: LocType,
        value_type: TypeEnum,
        size: SizeType,
        program: &'a Program,
    ) -> Self {
        Self {
            location,
            size,
            value_type,
            name,
            program,
        }
    }
}

impl Uniform<'_> {
    pub fn size(&self) -> SizeType {
        self.size
    }

    pub fn value_type(&self) -> TypeEnum {
        self.value_type
    }

    fn type_error(&self, current_type: &str) -> ! {
        log::error!(
            "Attempting to set uniform `{name}` with value of type {current_type}, but its type is {value_type}",
            name = self.name,
            value_type=self.value_type,
        );
        panic!("Type error while setting uniform")
    }

    pub fn set_mat4(&mut self, mat: &Matrix4<f32>) {
        if self.value_type != gl::FLOAT_MAT4 {
            self.type_error("mat4");
        }
        let a = AsRef::<[_; 16]>::as_ref(mat);
        unsafe {
            gl::ProgramUniformMatrix4fv(self.program.id, self.location as _, 1, gl::FALSE, a as _);
            gl_check!();
        }
    }

    pub fn set_float(&mut self, value: gl::types::GLfloat) {
        if self.value_type != gl::FLOAT {
            self.type_error("float");
        }
        unsafe {
            gl::ProgramUniform1f(self.program.id, self.location as _, value);
            gl_check!();
        }
    }

    pub fn set_vec3(&mut self, vec: &Vector3<f32>) {
        if self.value_type != gl::FLOAT_VEC3 {
            self.type_error("vec3");
        }
        unsafe {
            gl::ProgramUniform3f(self.program.id, self.location as _, vec.x, vec.y, vec.z);
            gl_check!();
        }
    }
    pub fn set_vec4(&mut self, vec: &Vector4<f32>) {
        if self.value_type != gl::FLOAT_VEC4 {
            self.type_error("vec4");
        }
        unsafe {
            gl::ProgramUniform4f(
                self.program.id,
                self.location as _,
                vec.x,
                vec.y,
                vec.z,
                vec.w,
            );
            gl_check!();
        }
    }

    pub fn set_int(&mut self, value: gl::types::GLint) {
        if self.value_type != gl::INT && self.value_type != gl::SAMPLER_2D {
            self.type_error("int or sampler2D")
        }
        unsafe {
            gl::ProgramUniform1i(self.program.id, self.location as _, value);
            gl_check!();
        }
    }
}
