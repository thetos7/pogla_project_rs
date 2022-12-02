extern crate gl;
extern crate sdl2;
extern crate ansi_term;

use engine::Engine;
use sdl2::{event::Event, keyboard::Keycode};
mod engine;
mod gl_check;

fn clear_frame() {}
const HEIGHT: i32 = 1024;
const WIDTH: i32 = 1024;

fn main() {
    let engine = unsafe { Engine::instance() };

    engine.init();

    let mut should_close = false;

    while !should_close {
        engine
            .update(&mut should_close) // update objects and handle events
            .display() // Draw objects to window
            .swap_buffer();
    }
}
