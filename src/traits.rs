pub trait Drawable {
    fn draw(&self);
}

pub trait Updateable {
    fn update(&mut self, delta_time: f32);
}
