use gl::types::GLfloat;

use crate::traits::ParticleLike;


#[repr(C)]
struct FireParticle {
    lifetime: GLfloat,
    scale: GLfloat,
    rotation: GLfloat,
    angular_velocity: GLfloat,
    position: [GLfloat; 3],
    velocity: [GLfloat; 3]
}

impl ParticleLike for FireParticle {
    fn setup_attributes() {
        todo!()
    }
}

