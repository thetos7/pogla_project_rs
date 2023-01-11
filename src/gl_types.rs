use gl::types::{GLenum, GLuint};

pub type VaoIdType = GLuint;
pub type BufferIdType = GLuint;

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
