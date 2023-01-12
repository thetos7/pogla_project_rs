#![allow(dead_code)]

extern crate ansi_term;
extern crate cgmath;
extern crate env_logger;
extern crate gl;
extern crate log;
extern crate sdl2;
extern crate rand;

use clap::Parser;
use engine::Engine;
mod definitions;
mod engine;
mod gl_check;
mod gl_utils;
mod input;
mod logger;
mod objects;
mod program;
mod traits;
mod gl_types;
mod extensions;
mod particles;

#[derive(Parser)]
#[command(name = "POGLA project")]
#[command(author = "Ancelin BOUCHET & Thibault AMBROSINO")]
#[command(version = "0.1.0")]
struct Args {
    #[arg(short = 'b', long)]
    /// Whether or not the sdl2 relative mouse mode implementation is broken (it is when you can move you mouse freely after clicking on the window)
    relative_mouse_broken: bool,
}

fn main() {
    logger::init_default();

    let args = Args::parse();

    unsafe { engine::BROKEN_RELATIVE_MOUSE_MODE = args.relative_mouse_broken };

    let engine = unsafe { Engine::instance_mut() };

    engine.init();

    let mut should_close = false;

    while !should_close {
        engine
            .update(&mut should_close) // update objects and handle events
            .display() // Draw objects to window
            .swap_buffer();
    }
}
