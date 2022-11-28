extern crate gl;
extern crate sdl2;

use sdl2::{event::Event, keyboard::Keycode};

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("POGLA project", 1024, 1024)
        .resizable()
        .opengl()
        .build()
        .unwrap();
    
    let _gl_context = window.gl_create_context().unwrap();
    
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::ClearColor(1.,0.,0.,1.);
    }

    let mut pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. }  => break 'main,
                _ => {}
            }
        }

        unsafe{ gl::Clear(gl::COLOR_BUFFER_BIT) }

        window.gl_swap_window();
    }
}
