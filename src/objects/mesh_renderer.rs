use std::{cell::RefCell, collections::HashMap, rc::Rc};

use cgmath::Matrix4;
use gl::types::{GLenum, GLfloat, GLint, GLuint};

use crate::{gl_checked, program::uniform::Uniform, traits::Drawable};

use self::builder::MeshRendererBuilder;
use crate::program::ProgramType;

type BufferType = Vec<GLfloat>;
type VaoType = GLuint;
type DrawModeType = GLenum;
type UniformType = Option<Rc<RefCell<Uniform>>>;
pub type MeshRendererPointer = Rc<RefCell<MeshRenderer>>;

#[derive(Default)]
pub enum DrawMode {
    #[default]
    Triangles,
    TrianglesAdjacency,
    TriangleStrip,
    TriangleStripAdjacency,
    TriangleFan,
    Lines,
    LinesAdjacency,
    LineStrip,
    LineStripAdjacency,
    LineLoop,
    Points,
    Patches,
}

impl DrawMode {
    pub fn gl_constant(&self) -> GLenum {
        match *self {
            Self::Triangles => gl::TRIANGLES,
            Self::TrianglesAdjacency => gl::TRIANGLES_ADJACENCY,
            Self::TriangleStrip => gl::TRIANGLE_STRIP,
            Self::TriangleStripAdjacency => gl::TRIANGLE_STRIP_ADJACENCY,
            Self::TriangleFan => gl::TRIANGLE_FAN,
            Self::Lines => gl::LINES,
            Self::LinesAdjacency => gl::LINES_ADJACENCY,
            Self::LineStrip => gl::LINE_STRIP,
            Self::LineStripAdjacency => gl::LINE_STRIP_ADJACENCY,
            Self::LineLoop => gl::LINE_LOOP,
            Self::Points => gl::POINTS,
            Self::Patches => gl::PATCHES,
        }
    }
}

pub struct MeshRenderer {
    shader: ProgramType,
    vao_id: VaoType,
    draw_mode: DrawModeType,
    vertex_count: usize,
    buffer_ids: Vec<GLuint>,
    transform: Matrix4<GLfloat>,
    transform_uniform: UniformType,
}

impl MeshRenderer {
    pub fn builder() -> MeshRendererBuilder {
        Default::default()
    }
}

mod builder {
    use std::mem::size_of;

    use cgmath::{Matrix4, SquareMatrix};
    use gl::types::GLuint;

    use crate::{definitions, gl_check, gl_checked, program::uniform};

    use super::*;

    type BufferCollectionType = Vec<BufferType>;
    type AttributeConfigType = (String, GLint);
    type AttributeConfigCollection = HashMap<usize, Vec<AttributeConfigType>>;
    #[derive(Default)]
    pub struct MeshRendererBuilder {
        buffers: BufferCollectionType,
        shader: Option<ProgramType>,
        draw_mode: Option<DrawMode>,
        attribute_config: AttributeConfigCollection,
        transform: Option<Matrix4<GLfloat>>,
    }

    impl MeshRendererBuilder {
        /// Adds a buffer
        ///
        /// The buffer will be given an internal ID starting at 0 and counting up with each buffer added,
        /// so first buffer has ID 0, second had ID 1, etc.
        pub fn add_buffer(mut self, buffer: BufferType) -> Self {
            self.buffers.push(buffer);
            self
        }

        /// Specifies the program to use when rendering the mesh
        pub fn shader(mut self, program: ProgramType) -> Self {
            self.shader = Some(program);
            self
        }

        /// Specifies how OpenGL should draw the mesh based on the contents of its buffer(s)
        pub fn draw_mode(mut self, mode: DrawMode) -> Self {
            self.draw_mode = Some(mode);
            self
        }

        /// Add an attribute to builder
        ///
        /// `name` refers to the attribute name in the shader code\
        /// `size` is the size in amount of values per attribute (vec3 has 3 fields, so size is 3)\
        /// `buffer_id` refers to the id of a buffer specified when using the builder
        pub fn add_attribute(
            mut self,
            name: impl Into<String>,
            size: GLint,
            buffer_id: usize,
        ) -> Self {
            if !self.attribute_config.contains_key(&buffer_id) {
                self.attribute_config.insert(buffer_id, Vec::new());
            }

            self.attribute_config
                .get_mut(&buffer_id)
                .unwrap()
                .push((name.into(), size));
            self
        }

        pub fn transform(mut self, mat: Matrix4<GLfloat>) -> Self {
            self.transform = Some(mat);
            self
        }

        pub fn build(self) -> MeshRendererPointer {
            self.assert_integrity();

            let mut vao_id: VaoType = 0;
            let program = self.shader.as_ref().unwrap();
            let program = program.as_ref().borrow();
            let prog_id = program.id();

            unsafe {
                gl_checked! {
                    gl::GenVertexArrays(1, &mut vao_id); // generate VAO
                    gl::BindVertexArray(vao_id);
                };
            }

            let mut buffer_ids: Vec<GLuint> = vec![0; self.buffers.len()];

            unsafe {
                gl::GenBuffers(self.buffers.len() as _, buffer_ids.as_mut_ptr()); // Generate buffers
                gl_check!();
            }

            let mut strides: Vec<usize> = vec![0; buffer_ids.len()];
            // for each buffer
            for i in 0..buffer_ids.len() {
                unsafe {
                    gl_checked! {
                        gl::BindBuffer(gl::ARRAY_BUFFER, buffer_ids[i]);
                        gl::NamedBufferData( // Load buffer data into OpenGL buffer
                            buffer_ids[i],
                            (self.buffers[i].len() * size_of::<GLfloat>()) as _,
                            self.buffers[i].as_ptr() as _,
                            gl::STATIC_DRAW,
                        );
                    };
                }

                // compute actual stride of each vertex by summing the individual attributes' sizes
                let mut stride = 0usize;
                for (_, size) in self.attribute_config.get(&i).unwrap().iter() {
                    stride += (*size) as usize;
                }

                strides[i] = stride;

                // configure attributes
                let offset = 0usize;
                for (name, size) in self.attribute_config.get(&i).unwrap().iter() {
                    let location = unsafe { gl::GetAttribLocation(prog_id, name.as_ptr() as _) };
                    gl_check!();
                    if location == -1 {
                        log::error!(
                            "attribute `{name}` could not be found in program `{program_name}`",
                            program_name = program.name()
                        );
                        continue;
                    }

                    unsafe {
                        gl_checked! {
                            gl::VertexAttribPointer(
                                location as _,
                                *size,
                                gl::FLOAT,
                                gl::FALSE,
                                (stride * size_of::<GLfloat>()) as _,
                                offset as _,
                            );
                            gl::EnableVertexArrayAttrib(vao_id, location as _);
                        };
                    }
                }
            }

            unsafe {
                gl_checked! {
                    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                    gl::BindVertexArray(0);
                };
            }

            Rc::new(RefCell::new(MeshRenderer {
                shader: self.shader.as_ref().unwrap().clone(),
                vao_id,
                draw_mode: self.draw_mode.unwrap_or_default().gl_constant(),
                vertex_count: self.buffers[0].len() / strides[0],
                buffer_ids,
                transform: self.transform.unwrap_or_else(Matrix4::identity),
                transform_uniform: program
                    .uniform(definitions::MODEL_TRANSFORM_UNIFORM_NAME)
                    .map(|p| p.clone()),
            }))
        }

        fn assert_integrity(&self) {
            let mut error = false;
            if self.buffers.is_empty() {
                log::error!("mesh renderer builder has no vertex buffers");
                error = true;
            }
            if self.shader.is_none() {
                log::error!("mesh renderer builder has no program");
                error = true;
            } else {
                let program = self.shader.as_ref().unwrap().as_ref().borrow();
                let transform_uniform = program.uniform(definitions::MODEL_TRANSFORM_UNIFORM_NAME);
                if transform_uniform.is_none() {
                    log::warn!(
                        "mesh renderer shader does not have a valid active model transform uniform"
                    )
                } else {
                    let uniform_type = transform_uniform.unwrap().as_ref().borrow().value_type();
                    if uniform_type != gl::FLOAT_MAT4 {
                        log::warn!(
                            "mesh renderer shader's model tranform has type {ty} instead of `mat4`",
                            ty = uniform::stringify_type(uniform_type)
                        )
                    }
                }
            }
            if self.attribute_config.is_empty() {
                log::error!("No attribute has been specified in mesh renderer builder.");
                error = true;
            }

            let mut missing_attribute_configs = false;
            for i in 0..self.buffers.len() {
                if !self.attribute_config.contains_key(&i) {
                    missing_attribute_configs = true;
                    log::debug!("missing attribute config for buffer {i} in mesh renderer builder");
                }
            }
            if missing_attribute_configs {
                log::error!("Some buffers are declared in mesh renderer builder, but no attribute have been defined for them (see debug logs)");
                error = true;
            }
            if error {
                panic!("Errors have occured while building mesh renderer, check logs");
            }
        }
    }
}

impl Drop for MeshRenderer {
    fn drop(&mut self) {
        unsafe {
            gl_checked! {
                gl::DeleteBuffers(self.buffer_ids.len() as _, self.buffer_ids.as_ptr());
                gl::DeleteVertexArrays(1, &self.vao_id);
            };
        }
    }
}

impl Drawable for MeshRenderer {
    fn draw(&self) {
        self.shader.as_ref().borrow().bind();
        if let Some(transform_uniform) = self.transform_uniform.as_ref() {
            transform_uniform
                .as_ref()
                .borrow_mut()
                .set_mat4(&self.transform);
        }

        unsafe {
            gl_checked! {
                gl::BindVertexArray(self.vao_id);
                gl::DrawArrays(self.draw_mode, 0, self.vertex_count as _);
                gl::BindVertexArray(0);
            };
        }
    }
}
