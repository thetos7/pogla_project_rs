use std::collections::HashMap;

use sdl2::{event::Event, keyboard::Keycode, video::Window, EventPump};

use crate::{
    gl_checked,
    program::{
        shader::{Shader, ShaderType},
        Program,
    },
};
#[derive(Default)]
pub struct Engine {
    sdl: Option<sdl2::Sdl>,
    video_subsystem: Option<sdl2::VideoSubsystem>,
    window: Option<Window>,
    _gl_context: Option<sdl2::video::GLContext>,
    pump: Option<EventPump>,
    programs: HashMap<String, Program>,
}

static mut INSTANCE: Option<Engine> = None;

impl Engine {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub unsafe fn instance() -> &'static mut Self {
        if let None = INSTANCE {
            INSTANCE = Some(Engine::new());
        }
        INSTANCE.as_mut().unwrap_unchecked()
    }

    fn _init_sdl(&mut self) {
        let sdl = sdl2::init().expect("Couldn't initialize SDL2.");
        let video_subsystem = sdl.video().expect("Couldn't create video system.");
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 5);

        let window = video_subsystem
            .window("POGLA project", 1024, 1024)
            .resizable()
            .opengl()
            .build()
            .expect("Couldn't create window");

        let _gl_context = window
            .gl_create_context()
            .expect("Couldn't create openGL context");

        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        let pump = sdl
            .event_pump()
            .expect("SDL Event pump creation failed, one may already exist.");

        self.sdl = Some(sdl);
        self.video_subsystem = Some(video_subsystem);
        self.window = Some(window);
        self.pump = Some(pump);
        self._gl_context = Some(_gl_context);
    }

    fn _init_gl(&mut self) {
        unsafe {
            const PIXEL_BYTE_ALIGNMENT_LEN: i32 = 1; // 1 byte for 8-bit deep color
            gl_checked! {
                gl::Enable(gl::DEPTH_TEST);
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                // gl::Enable(gl::CULL_FACE);
                gl::ClearColor(0.66, 0.66, 0.66, 1.);
                gl::PixelStorei(gl::PACK_ALIGNMENT, PIXEL_BYTE_ALIGNMENT_LEN);
                gl::PixelStorei(gl::UNPACK_ALIGNMENT, PIXEL_BYTE_ALIGNMENT_LEN);
            };
        }
    }

    fn _init_shaders(&mut self) {
        let program = Program::builder("basic")
            .add_shader(
                "basic_vertex",
                Shader::new(ShaderType::Vertex)
                    .load("resources/shaders/basic.vert")
                    .expect("Couldn't read basic_shader vertex file"),
            )
            .add_shader(
                "basic_frag",
                Shader::new(ShaderType::Fragment)
                    .load("resources/shaders/basic.frag")
                    .expect("Couldn't read basic_shader fragment file"),
            )
            .build();

        let program = match program {
            Ok(prog) => prog,
            Err(error) => {
                error.log_error();
                panic!("program creation failed");
            }
        };
        self.programs.insert("basic".into(), program);
    }

    pub fn init(&mut self) -> &mut Self {
        self._init_sdl();
        self._init_gl();
        self._init_shaders();
        self
    }

    fn _handle_events(&mut self, should_close: &mut bool) {
        for event in self.pump.as_mut().unwrap().poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    *should_close = true;
                    return;
                }
                _ => {}
            }
        }
    }

    pub fn update(&mut self, should_close: &mut bool) -> &mut Self {
        self._handle_events(should_close);
        if *should_close {
            return self;
        }
        self
    }

    fn _clear_frame(&self) {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) };
    }

    pub fn display(&self) -> &Self {
        self._clear_frame();
        self
    }
    pub fn swap_buffer(&self) -> &Self {
        self.window.as_ref().unwrap().gl_swap_window();
        self
    }
}
