extern crate ansi_term;
extern crate gl;
extern crate sdl2;

use engine::Engine;
mod engine;
mod gl_check;

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
