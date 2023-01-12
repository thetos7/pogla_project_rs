use std::mem::size_of;

use gl::types::GLfloat;

use crate::{gl_checked, traits::ParticleLike};

#[repr(C)]
#[derive(Debug)]
pub struct FireParticle {
    pub lifetime: GLfloat,
    pub rotation: GLfloat,
    pub position: [GLfloat; 3],
    pub angular_velocity: GLfloat,
    pub velocity: [GLfloat; 3],
}

impl ParticleLike for FireParticle {
    fn setup_attributes() {
        let size = size_of::<Self>();
        let float_size = size_of::<GLfloat>();
        unsafe {
            gl_checked! {
                gl::VertexAttribPointer(1, 1, gl::FLOAT, gl::FALSE, size as _, (0 * float_size) as _);
                gl::VertexAttribPointer(2, 1, gl::FLOAT, gl::FALSE, size as _, (1 * float_size) as _);
                gl::VertexAttribPointer(3, 3, gl::FLOAT, gl::FALSE, size as _, (2 * float_size) as _);
                gl::EnableVertexAttribArray(1);
                gl::EnableVertexAttribArray(2);
                gl::EnableVertexAttribArray(3);
            };
        }
    }
}
