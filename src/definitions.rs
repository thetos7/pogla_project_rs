use std::f32::consts::FRAC_PI_2;

use gl::types::GLfloat;

pub const MODEL_TRANSFORM_UNIFORM_NAME: &str = "model_transform";
pub const VIEW_TRANSFORM_UNIFORM_NAME: &str = "view_transform";
pub const PROJECTION_UNIFORM_NAME: &str = "projection";
pub const DEFAULT_FOV: f32 = FRAC_PI_2;
pub const DEFAULT_ZNEAR: f32 = 0.5;
pub const DEFAULT_ZFAR: f32 = 100.;
pub const DEFAULT_ASPECT_RATIO: f32 = 1.0;

pub const GLSL_VERSION_SRC: &'static str = "resources/shaders/version.glsl";

pub const CUBE_VERTICES_BUFFER: [GLfloat; 108] = [
    // face Up
    // tri1
    1.0, 1.0, 1.0, // v8
    -1.0, 1.0, 1.0, // v7
    1.0, -1.0, 1.0, // v6
    // tri2
    1.0, -1.0, 1.0, // v6
    -1.0, 1.0, 1.0, // v7
    -1.0, -1.0, 1.0, // v5
    // face Down
    // tri1
    1.0, -1.0, -1.0, // v2
    -1.0, 1.0, -1.0, // v3
    1.0, 1.0, -1.0, // v4
    // tri2
    -1.0, 1.0, -1.0, // v3
    1.0, -1.0, -1.0, // v2
    -1.0, -1.0, -1.0, // v1
    // face West
    // tri1
    -1.0, 1.0, 1.0, // v7
    1.0, 1.0, 1.0, // v8
    1.0, 1.0, -1.0, // v4
    // tri2
    -1.0, 1.0, 1.0, // v7
    1.0, 1.0, -1.0, // v4
    -1.0, 1.0, -1.0, // v3
    // face East
    // tri1
    1.0, -1.0, -1.0, // v2
    1.0, -1.0, 1.0, // v6
    -1.0, -1.0, 1.0, // v5
    // tri2
    -1.0, -1.0, -1.0, // v1
    1.0, -1.0, -1.0, // v2
    -1.0, -1.0, 1.0, // v5
    // face North
    // tri1
    1.0, -1.0, -1.0, // v1
    1.0, 1.0, 1.0, // v7
    1.0, -1.0, 1.0, // v6
    // tri2
    1.0, -1.0, -1.0, // v1
    1.0, 1.0, -1.0, // v3
    1.0, 1.0, 1.0, // v7
    // face South
    // tri1
    -1.0, -1.0, 1.0, // v5
    -1.0, 1.0, 1.0, // v7
    -1.0, -1.0, -1.0, // v1
    // tri2
    -1.0, 1.0, -1.0, // v3
    -1.0, -1.0, -1.0, // v1
    -1.0, 1.0, 1.0, // v7
];