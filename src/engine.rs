use sdl2::video::Window;

use crate::gl_check;

#[derive(Default)]
pub struct Engine {
    sdl: Option<sdl2::Sdl>,
    video_subsystem: Option<sdl2::VideoSubsystem>,
    window: Option<Window>,
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
            println!("only prints once!")
        }
        INSTANCE.as_mut().unwrap()
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

        let _ = window
            .gl_create_context()
            .expect("Couldn't create openGL context");

        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        self.sdl = Some(sdl);
        self.video_subsystem = Some(video_subsystem);
        self.window = Some(window);
    }

    fn _init_load_gl(&mut self) {
        let video_subsystem = self.video_subsystem.as_ref().unwrap();
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    }

    fn _init_gl(&mut self) {
        unsafe {
            const PIXEL_BYTE_ALIGNMENT_LEN: i32 = 1; // 1 byte for 8-bit deep color
            gl::Enable(gl::DEPTH_TEST);
            gl_check!();
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            gl_check!();
            // gl::Enable(gl::CULL_FACE);
            // gl_check!();
            gl::ClearColor(0.66, 0.66, 0.66, 1.);
            gl_check!();
            gl::PixelStorei(gl::PACK_ALIGNMENT, PIXEL_BYTE_ALIGNMENT_LEN);
            gl_check!();
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, PIXEL_BYTE_ALIGNMENT_LEN);
            gl_check!();
        }
    }

    pub fn init(&mut self) -> &mut Self {
        self._init_sdl();
        self._init_load_gl();
        self._init_gl();
        self
    }
}
