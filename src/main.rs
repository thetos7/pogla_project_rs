extern crate ansi_term;
extern crate env_logger;
extern crate gl;
extern crate log;
extern crate sdl2;

use engine::Engine;
mod engine;
mod gl_check;
mod program;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
mod gl_utils;

fn main() {
    Builder::new()
        .format(|buf, record| {
            write!(buf, "[{}]", record.level())?;
            if let (Some(file), Some(line)) = (record.file(), record.line()) {
                write!(buf, "({file}:{line})")?;
            }
            writeln!(buf, "{}", record.args())
        })
        .filter(None, LevelFilter::Info)
        .init();

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
