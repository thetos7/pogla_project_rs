use std::f32::consts::FRAC_PI_2;

pub const MODEL_TRANSFORM_UNIFORM_NAME: &str = "model_transform";
pub const VIEW_TRANSFORM_UNIFORM_NAME: &str = "view_transform";
pub const PROJECTION_UNIFORM_NAME: &str = "projection";
pub const DEFAULT_FOV: f32 = FRAC_PI_2;
pub const DEFAULT_ZNEAR: f32 = 0.5;
pub const DEFAULT_ZFAR: f32 = 100.;
pub const DEFAULT_ASPECT_RATIO: f32 = 1.0;
