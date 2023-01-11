use gl::types::GLuint;

use crate::{
    extensions::CeilDiv,
    gl_check, gl_checked,
    gl_types::{BufferIdType, DrawMode, VaoIdType},
    program::{Program, ProgramSharedPointer},
    traits::{Drawable, Particle, Updatable},
};

pub struct ParticleSystem {
    compute_program: Program,
    display_program: ProgramSharedPointer,
    vao_id: VaoIdType,
    buffer_id: BufferIdType,
    particle_count: usize,
    group_size: usize,
}

pub mod builder {
    use std::mem::size_of;

    use crate::{
        gl_check,
        program::{Program, ProgramSharedPointer},
        traits::Particle,
    };

    use super::*;

    const DEFAULT_GROUP_SIZE: usize = 1024;

    pub struct ParticleSystemBuilder<ParticleType: Particle> {
        compute_program: Option<Program>,
        display_program: Option<ProgramSharedPointer>,
        initial_particles: Option<Vec<ParticleType>>,
        group_size: Option<usize>,
        buffer_base: Option<GLuint>,
    }

    impl<T: Particle> Default for ParticleSystemBuilder<T> {
        fn default() -> Self {
            Self {
                compute_program: Default::default(),
                display_program: Default::default(),
                initial_particles: Default::default(),
                buffer_base: Default::default(),
                group_size: Default::default(),
            }
        }
    }

    impl<ParticleType: Particle> ParticleSystemBuilder<ParticleType> {
        pub fn compute_program(mut self, program: Program) -> Self {
            self.compute_program = Some(program);
            self
        }

        pub fn display_program(mut self, program: ProgramSharedPointer) -> Self {
            self.display_program = Some(program);
            self
        }

        pub fn initial_particles(mut self, particles: Vec<ParticleType>) -> Self {
            self.initial_particles = Some(particles);
            self
        }

        pub fn buffer_base(mut self, base_binding: GLuint) -> Self {
            self.buffer_base = Some(base_binding);
            self
        }

        pub fn group_size(mut self, group_size: usize) -> Self {
            self.group_size = Some(group_size);
            self
        }

        fn assert_integrity(&self) {
            let mut error = false;

            if let None = self.initial_particles {
                log::error!("Particle system's initial particles are missing");
                error = true;
            };

            if let Some(ref program) = self.compute_program {
                if !program.is_compute() {
                    log::error!(
                        "Particle system compute/simulation program is not a compute shader"
                    );
                    error = true;
                }
            } else {
                log::error!("Particle system's compute program missing");
                error = true;
            }

            if let Some(ref program) = self.display_program {
                if !program.borrow().has_geometry() {
                    log::warn!("Particle system's display program contains no geometry shader, this may be a mistake");
                }
            } else {
                log::error!("Particle system's display program missing");
                error = true;
            }

            if let Some(ref size) = self.group_size {
                if !(0..=1024).contains(size) {
                    log::error!("Particle system's group size must be in the range [0,1024]");
                    error = true;
                }
            }

            if error {
                panic!(
                    "particle system builder integrity assertion failed due to the previous errors"
                );
            }
        }

        pub fn build(self) -> ParticleSystem {
            self.assert_integrity();
            let particles = self.initial_particles.unwrap();
            let particle_byte_size = size_of::<ParticleType>();

            let mut vao_id: VaoIdType = 0;

            unsafe {
                gl_checked! {
                    gl::GenVertexArrays(1, &mut vao_id);
                    gl::BindVertexArray(vao_id);
                };
            }

            let mut buffer_id: BufferIdType = 0;
            let display_program = self.display_program.unwrap();

            unsafe {
                gl_check!();
                gl_checked! {
                    gl::GenBuffers(1, &mut buffer_id);
                    gl::NamedBufferData(
                        buffer_id,
                        (particles.len() * particle_byte_size) as _,
                        particles.as_ptr() as _,
                        gl::DYNAMIC_DRAW,
                    );
                    gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, buffer_id);
                    gl::BindBufferBase(
                        gl::SHADER_STORAGE_BUFFER,
                        self.buffer_base.unwrap_or(1),
                        buffer_id,
                    );
                    gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id);
                };
            }

            ParticleType::setup_attributes();

            unsafe {
                gl::BindVertexArray(0);
                gl_check!();
            }

            ParticleSystem {
                compute_program: self.compute_program.unwrap(),
                display_program,
                vao_id,
                buffer_id,
                particle_count: particles.len(),
                group_size: self.group_size.unwrap_or(DEFAULT_GROUP_SIZE),
            }
        }
    }
}

impl Drop for ParticleSystem {
    fn drop(&mut self) {
        unsafe {
            gl_checked! {
                gl::DeleteBuffers(1, &self.buffer_id);
                gl::DeleteVertexArrays(1, &self.vao_id);
            };
        }
    }
}

impl ParticleSystem {
    /// Creates a new particle system builder
    /// ParticleType should have a C representation (add attribute `#[repr(C)]` to struct)
    pub fn builder<ParticleType: Particle>() -> builder::ParticleSystemBuilder<ParticleType> {
        Default::default()
    }
}

impl Drawable for ParticleSystem {
    fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao_id);
            gl_check!();
        }

        let _ctx = self.display_program.borrow().bound_context();

        unsafe {
            gl_checked! {
                gl::DrawArrays(DrawMode::Points.gl_constant(), 0, self.particle_count as _);
                gl::BindVertexArray(0);
            };
        }
    }
}

impl Updatable for ParticleSystem {
    fn update(&mut self, delta_time: f32) {
        unsafe {
            gl::BindVertexArray(self.vao_id);
            gl_check!();
        }

        if let Some(uniform) = self.compute_program.uniform("uDeltaTime") {
            uniform.borrow_mut().set_float(delta_time);
        }

        let _ctx = self.compute_program.bound_context();

        let group_count: GLuint = self.particle_count.ceil_div(self.group_size) as _;

        unsafe {
            gl_checked! {
                gl::DispatchCompute(group_count, 1, 1);
                gl::MemoryBarrier(gl::ALL_BARRIER_BITS);
                gl::BindVertexArray(0);
            };
        }
    }
}
