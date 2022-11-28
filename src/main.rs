extern crate gl;
extern crate sdl2;

use sdl2::{event::Event, keyboard::Keycode};

fn clear_frame() {
    unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
}

fn main() {
    let sdl = sdl2::init().expect("Couldn't initialize SDL2.");
    let video_subsystem = sdl.video().expect("Couldn't create video system.");
    let window = video_subsystem
        .window("POGLA project", 1024, 1024)
        .resizable()
        .opengl()
        .build()
        .expect("Couldn't create window");

    let _gl_context = window
        .gl_create_context()
        .expect("Couldn't create openGL context");
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe { gl::ClearColor(0.66, 0.66, 0.66, 1.) };

    let mut pump = sdl
        .event_pump()
        .expect("SDL Event pump creation failed, one may already exist.");
    'main: loop {
        for event in pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
        }

        clear_frame();

        window.gl_swap_window();
    }
}
