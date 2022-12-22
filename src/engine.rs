use std::{cell::RefCell, collections::HashMap, rc::Rc, time::Instant};

use cgmath::{Matrix4, SquareMatrix, Vector4};
use gl::types::GLfloat;
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    mouse::MouseButton,
    video::Window,
    EventPump,
};

use crate::{
    definitions, gl_checked,
    input::InputState,
    objects::{Camera, DrawMode, MeshRenderer},
    program::{
        shader::{Shader, ShaderType},
        Program,
    },
    traits::{Drawable, Updateable},
};

const VERTICES: [GLfloat; 9] = [0., 0.5, 0., -0.5, -0.5, 0., 0.5, -0.5, 0.];

#[derive(Default)]
pub struct Engine {
    sdl: Option<sdl2::Sdl>,
    video_subsystem: Option<sdl2::VideoSubsystem>,
    window: Option<Window>,
    _gl_context: Option<sdl2::video::GLContext>,
    pump: Option<EventPump>,
    programs: HashMap<String, Rc<RefCell<Program>>>,
    last_frame_time: Option<Instant>,
    updateables: Vec<Rc<RefCell<dyn Updateable>>>,
    drawables: Vec<Rc<RefCell<dyn Drawable>>>,
}

static mut INSTANCE: Option<Engine> = None;

impl Engine {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub unsafe fn instance_mut() -> &'static mut Self {
        INSTANCE.get_or_insert_with(Engine::new)
    }

    pub unsafe fn instance() -> &'static Self {
        INSTANCE.get_or_insert_with(Engine::new)
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
                gl::FrontFace(gl::CCW);
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

        let program = Program::builder("uniform")
            .add_shader(
                "uniform_vertex",
                Shader::new(ShaderType::Vertex)
                    .load("resources/shaders/uniform/uniform.vert")
                    .unwrap(),
            )
            .add_shader(
                "uniform_fragment",
                Shader::new(ShaderType::Fragment)
                    .load("resources/shaders/uniform/uniform.frag")
                    .unwrap(),
            )
            .build();

        let program = match program {
            Ok(prog) => prog,
            Err(error) => {
                error.log_error();
                panic!("program creation failed");
            }
        };

        program
            .borrow()
            .uniform("projection")
            .unwrap()
            .borrow_mut()
            .set_mat4(&Matrix4::identity());

        program
            .borrow()
            .uniform("view_transform")
            .unwrap()
            .borrow_mut()
            .set_mat4(&Matrix4::identity());

        program
            .borrow()
            .uniform("object_color")
            .unwrap()
            .borrow_mut()
            .set_vec4(&Vector4::new(1., 0., 0., 1.));

        self.programs.insert("uniform".into(), program);
    }

    fn _init_objects(&mut self) {
        let triangle_renderer = MeshRenderer::builder()
            .shader(self.programs.get("uniform").unwrap().clone())
            .add_buffer(Vec::from(VERTICES.as_slice()))
            .add_attribute("position", 3, 0)
            .draw_mode(DrawMode::Triangles)
            .build();
        self.drawables.push(triangle_renderer);
    }

    pub fn init(&mut self) -> &mut Self {
        self._init_sdl();
        self._init_gl();
        self._init_shaders();
        self._init_objects();
        self
    }

    fn _handle_events(&mut self, should_close: &mut bool) {
        static mut PREV_MOUSE_X: i32 = 0;
        static mut PREV_MOUSE_Y: i32 = 0;

        let mut input = unsafe { InputState::get_mut() };
        input.mouse_x_axis = 0.; // reset mouse, no movement = no event
        input.mouse_y_axis = 0.;

        let events = self.pump.as_mut().unwrap();

        for event in events.poll_iter() {
            match event {
                // quit event has no window id
                Event::Quit { .. } => {
                    *should_close = true;
                    return;
                }
                _ => {}
            }

            let Some(window_id) = event.get_window_id() else {continue};
            if window_id != self.window.as_ref().unwrap().id() {
                continue;
            }

            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    *should_close = true;
                    return;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Tab),
                    ..
                } => {
                    self.sdl
                        .as_ref()
                        .unwrap()
                        .mouse()
                        .set_relative_mouse_mode(false);
                    input.capture_cursor = false;
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    // may need more checks
                    self.sdl
                        .as_ref()
                        .unwrap()
                        .mouse()
                        .set_relative_mouse_mode(true);
                    input.capture_cursor = true;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => input.forward = true,

                Event::KeyUp {
                    keycode: Some(Keycode::Z),
                    ..
                } => input.forward = false,

                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => input.backward = true,

                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => input.backward = false,

                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => input.right = true,

                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => input.right = false,

                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => input.left = true,

                Event::KeyUp {
                    keycode: Some(Keycode::Q),
                    ..
                } => input.left = false,

                Event::KeyDown {
                    keycode: Some(Keycode::LShift),
                    ..
                } => input.down = true,

                Event::KeyUp {
                    keycode: Some(Keycode::LShift),
                    ..
                } => input.down = false,

                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => input.up = true,

                Event::KeyUp {
                    keycode: Some(Keycode::Space),
                    ..
                } => input.up = false,

                Event::KeyDown {
                    keycode: Some(Keycode::L),
                    ..
                } => {
                    let camera = self.main_camera.as_ref().unwrap().borrow();
                    log::debug!("camera forward: {:#?}", camera.forward());
                    log::debug!("camera position: {:#?}", camera.position());
                    log::debug!("camera projection: {:#?}", camera.projection());
                    log::debug!("camera view_transform: {:#?}", camera.transform());
                }

                Event::Window {
                    win_event: WindowEvent::FocusGained,
                    ..
                } => input.focused = true,

                Event::Window {
                    win_event: WindowEvent::FocusLost,
                    ..
                } => input.focused = false,

                Event::MouseMotion {
                    xrel: x, yrel: y, ..
                } => unsafe {
                    input.mouse_x_axis = (x - PREV_MOUSE_X) as f32;
                    input.mouse_y_axis = (y - PREV_MOUSE_Y) as f32;
                    PREV_MOUSE_X = x;
                    PREV_MOUSE_Y = y;
                },

                _ => {}
            }
        }
    }

    pub fn update(&mut self, should_close: &mut bool) -> &mut Self {
        self._handle_events(should_close);
        if *should_close {
            return self;
        }

        let delta = if let Some(instant) = self.last_frame_time {
            instant.elapsed().as_secs_f32()
        } else {
            0f32
        };

        self.last_frame_time = Some(Instant::now());

        for item in self.updateables.iter_mut() {
            item.borrow_mut().update(delta)
        }

        self
    }

    fn _clear_frame(&self) {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) };
    }

    pub fn display(&self) -> &Self {
        self._clear_frame();

        for item in self.drawables.iter() {
            item.borrow().draw();
        }

        self
    }
    pub fn swap_buffer(&self) -> &Self {
        self.window.as_ref().unwrap().gl_swap_window();
        self
    }
}
