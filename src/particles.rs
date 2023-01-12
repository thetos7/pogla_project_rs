use std::mem::size_of;

use gl::types::GLfloat;

use crate::{gl_checked, traits::ParticleLike};

#[repr(C, align(16))]
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
        unsafe {
            gl_checked! {
                gl::VertexAttribPointer(1, 1, gl::FLOAT, gl::FALSE, size as _, 0 as _);
                gl::VertexAttribPointer(2, 1, gl::FLOAT, gl::FALSE, size as _, 4 as _);
                gl::VertexAttribPointer(3, 3, gl::FLOAT, gl::FALSE, size as _, 8 as _);
                gl::VertexAttribPointer(4, 1, gl::FLOAT, gl::FALSE, size as _, 20 as _);
                gl::VertexAttribPointer(5, 3, gl::FLOAT, gl::FALSE, size as _, 24 as _);
                gl::EnableVertexAttribArray(1);
                gl::EnableVertexAttribArray(2);
                gl::EnableVertexAttribArray(3);
                gl::EnableVertexAttribArray(4);
                gl::EnableVertexAttribArray(5);
            };
        }
    }
}
