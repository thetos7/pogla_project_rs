use std::{mem::size_of, f32::consts::{PI, FRAC_PI_4}};

use cgmath::Vector3;
use gl::types::GLfloat;
use rand::Rng;

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

impl FireParticle {
    pub fn spawn(count: usize) -> Vec<Self> {
        let mut particles = vec![];
        
        let mut rng = rand::thread_rng();

        for _ in 0..count {
            let lifetime = rng.gen_range(0.0..4.0);

            let yaw = rng.gen_range(0.0..(2.0 * PI));
            let hor_scale = rng.gen_range(0.5..1.0);
            let vert_scale = rng.gen_range(1.0..2.0);
            let velocity = Vector3::new(yaw.sin() * hor_scale, yaw.cos() * hor_scale, vert_scale);

            let position = Vector3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);

            particles.push(FireParticle {
                lifetime,
                velocity: velocity.into(),
                position: position.clone().into(),
                angular_velocity: rng.gen_range(-FRAC_PI_4..FRAC_PI_4),
                rotation: rng.gen_range(0.0..(2.0 * PI)),
            });
        }
        particles
    }
}
