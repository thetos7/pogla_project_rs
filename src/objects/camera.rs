use std::f32::consts::{FRAC_PI_2, PI};

use cgmath::{InnerSpace, Matrix4, Point3, Vector3};
use gl::types::GLfloat;

use crate::{input::InputState, traits::Updatable};

const UP: Vector3<GLfloat> = Vector3::new(0., 0., 1.);
const LOOK_SENSITIVITY: f32 = 0.005;
const SPEED: f32 = 3.0;

pub struct Camera {
    position: Point3<GLfloat>,
    pitch: GLfloat,
    yaw: GLfloat,
    projection: Matrix4<GLfloat>,
}

fn safe_normalize(vector: Vector3<GLfloat>) -> Vector3<GLfloat> {
    let norm = vector.magnitude();
    if norm == 0. {
        vector
    } else {
        vector.normalize()
    }
}

impl Camera {
    pub fn new(
        position: Point3<GLfloat>,
        pitch: GLfloat,
        yaw: GLfloat,
        projection: Matrix4<GLfloat>,
    ) -> Self {
        Camera {
            position,
            pitch,
            yaw,
            projection,
        }
    }

    pub fn forward(&self) -> Vector3<GLfloat> {
        Vector3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.yaw.sin() * self.pitch.cos(),
            self.pitch.sin(),
        )
    }

    /// forward unit vector on the xy (horizontal) plane
    pub fn planar_forward(&self) -> Vector3<GLfloat> {
        Vector3::new(self.yaw.cos(), self.yaw.sin(), 0.)
    }

    pub fn position(&self) -> &Point3<GLfloat> {
        &self.position
    }

    pub fn transform(&self) -> Matrix4<GLfloat> {
        let forward = self.forward();
        let dot = forward.dot(UP);
        let up = if dot < -0.99 {
            self.planar_forward()
        } else if dot > 0.99 {
            -self.planar_forward()
        } else {
            UP
        };
        Matrix4::look_to_rh(self.position, forward, up)
    }

    pub fn projection(&self) -> &Matrix4<GLfloat> {
        &self.projection
    }

    pub fn set_projection(&mut self, projection: Matrix4<GLfloat>) {
        self.projection = projection;
    }

    pub fn move_relative(&mut self, movement: &Vector3<GLfloat>) -> &mut Self {
        let forward = self.planar_forward();
        let right = Vector3::new(
            (self.yaw - FRAC_PI_2).cos(),
            (self.yaw - FRAC_PI_2).sin(),
            0.,
        );
        let global_movement = movement.x * forward + movement.y * right + movement.z * UP;
        self.position += global_movement;
        self
    }
}

impl Updatable for Camera {
    fn update(&mut self, delta_time: f32) {
        let input = unsafe { InputState::get() };
        if !input.focused {
            return;
        }

        if input.capture_cursor {
            // update pitch
            let pitch_movement = -input.mouse_y_axis * LOOK_SENSITIVITY * PI * 2.;
            self.pitch += pitch_movement;
            self.pitch = self.pitch.clamp(-FRAC_PI_2, FRAC_PI_2);
            // update yaw
            let yaw_movement = -input.mouse_x_axis * LOOK_SENSITIVITY * PI * 2.;
            self.yaw += yaw_movement;
            self.yaw %= PI * 2.;
        }

        // update position
        let x_input = input.forward as i32 - input.backward as i32;
        let y_input = input.right as i32 - input.left as i32;
        let z_input = input.up as i32 - input.down as i32;
        let move_direction =
            safe_normalize(Vector3::new(x_input as f32, y_input as f32, z_input as f32));
        let movement = move_direction * SPEED * delta_time;
        self.move_relative(&movement);
    }
}
