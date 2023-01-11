pub trait Drawable {
    fn draw(&self);
}

pub trait Updatable {
    fn update(&mut self, delta_time: f32);
}

pub trait Particle {
    fn setup_attributes();
}
