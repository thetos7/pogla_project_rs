use std::{cell::RefCell, collections::HashMap, f32::consts::PI, rc::Rc, time::Instant};

use cgmath::{Matrix4, PerspectiveFov, Point3, Rad, SquareMatrix, Vector4};
use gl::types::GLfloat;
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    mouse::MouseButton,
    video::Window,
    EventPump,
};

use crate::{
    definitions, gl_check, gl_checked,
    input::InputState,
    objects::{Camera, DrawMode, MeshRenderer},
    program::{
        shader::{Shader, ShaderType},
        uniform::Uniform,
        Program,
    },
    traits::{Drawable, Updateable},
};

const VERTICES: [GLfloat; 108] = [
    // face Up
    // tri1
    1.0, 1.0, 1.0, // v8
    -1.0, 1.0, 1.0, // v7
    1.0, -1.0, 1.0, // v6
    // tri2
    1.0, -1.0, 1.0, // v6
    -1.0, 1.0, 1.0, // v7
    -1.0, -1.0, 1.0, // v5
    // face Down
    // tri1
    1.0, -1.0, -1.0, // v2
    -1.0, 1.0, -1.0, // v3
    1.0, 1.0, -1.0, // v4
    // tri2
    -1.0, 1.0, -1.0, // v3
    1.0, -1.0, -1.0, // v2
    -1.0, -1.0, -1.0, // v1
    // face West
    // tri1
    -1.0, 1.0, 1.0, // v7
    1.0, 1.0, 1.0, // v8
    1.0, 1.0, -1.0, // v4
    // tri2
    -1.0, 1.0, 1.0, // v7
    1.0, 1.0, -1.0, // v4
    -1.0, 1.0, -1.0, // v3
    // face East
    // tri1
    1.0, -1.0, -1.0, // v2
    1.0, -1.0, 1.0, // v6
    -1.0, -1.0, 1.0, // v5
    // tri2
    -1.0, -1.0, -1.0, // v1
    1.0, -1.0, -1.0, // v2
    -1.0, -1.0, 1.0, // v5
    // face North
    // tri1
    1.0, -1.0, -1.0, // v1
    1.0, 1.0, 1.0, // v7
    1.0, -1.0, 1.0, // v6
    // tri2
    1.0, -1.0, -1.0, // v1
    1.0, 1.0, -1.0, // v3
    1.0, 1.0, 1.0, // v7
    // face South
    // tri1
    -1.0, -1.0, 1.0, // v5
    -1.0, 1.0, 1.0, // v7
    -1.0, -1.0, -1.0, // v1
    // tri2
    -1.0, 1.0, -1.0, // v3
    -1.0, -1.0, -1.0, // v1
    -1.0, 1.0, 1.0, // v7
];

type UniformCollection = Vec<Rc<RefCell<Uniform>>>;
type CameraPointer = Rc<RefCell<Camera>>;

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
    view_transform_uniforms: UniformCollection,
    projection_uniforms: UniformCollection,
    main_camera: Option<CameraPointer>,
}

static mut INSTANCE: Option<Engine> = None;

pub static mut BROKEN_RELATIVE_MOUSE_MODE: bool = false;

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

        self.register_program("basic", program);

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

        self.register_program("uniform", program);
    }

    fn _init_objects(&mut self) {
        let triangle_renderer = MeshRenderer::builder()
            .shader(self.programs.get("uniform").unwrap().clone())
            .add_buffer(Vec::from(VERTICES.as_slice()))
            .add_attribute("position", 3, 0)
            .draw_mode(DrawMode::Triangles)
            .build();
        self.register_renderer(triangle_renderer);
    }

    fn register_dynamic_object(&mut self, obj: Rc<RefCell<dyn Updateable>>) {
        self.updateables.push(obj);
    }

    fn register_renderer(&mut self, obj: Rc<RefCell<dyn Drawable>>) {
        self.drawables.push(obj);
    }

    fn register_program(&mut self, name: impl Into<String>, program: Rc<RefCell<Program>>) {
        let name = name.into();
        self.programs.insert(name.clone(), program.clone());

        let program = program.as_ref().borrow();

        let projection = program.uniform(definitions::PROJECTION_UNIFORM_NAME);

        let view_transform = program.uniform(definitions::VIEW_TRANSFORM_UNIFORM_NAME);

        let mut error: bool = false;

        let has_projection = projection.is_some();
        let is_projection_type_ok = if let Some(u) = projection {
            u.as_ref().borrow().value_type() == gl::FLOAT_MAT4
        } else {
            false
        };

        let has_view_transform = view_transform.is_some();
        let is_view_transform_type_ok = if let Some(u) = view_transform {
            u.as_ref().borrow().value_type() == gl::FLOAT_MAT4
        } else {
            false
        };

        if has_projection != has_view_transform && (has_projection || has_view_transform) {
            log::warn!(
                "program `{name}` has one of {projection} or {view_transform} uniforms, but not both.",
                projection = definitions::PROJECTION_UNIFORM_NAME,
                view_transform = definitions::VIEW_TRANSFORM_UNIFORM_NAME
            );
            error = true;
        }

        if has_projection && !is_projection_type_ok {
            log::warn!(
                "The {projection} uniform of program `{name}` is not of type mat4",
                projection = definitions::PROJECTION_UNIFORM_NAME
            );
            error = true;
        }
        if has_view_transform && !is_view_transform_type_ok {
            log::warn!(
                "The {view_transform} uniform of program `{name}` is not of type mat4",
                view_transform = definitions::VIEW_TRANSFORM_UNIFORM_NAME
            );
            error = true;
        }

        // view independant shader
        if !(has_projection || has_view_transform) {
            return;
        }

        if !error {
            self.view_transform_uniforms
                .push(view_transform.unwrap().clone());
            self.projection_uniforms.push(projection.unwrap().clone());
        } else {
            log::warn!("Due to the previous uniform error(s), the program `{name}` will not respond to camera changes")
        }
    }

    fn _init_point_of_view(&mut self) {
        let projection = Matrix4::from(PerspectiveFov {
            aspect: definitions::DEFAULT_ASPECT_RATIO,
            fovy: Rad(definitions::DEFAULT_FOV),
            near: definitions::DEFAULT_ZNEAR,
            far: definitions::DEFAULT_ZFAR,
        });

        let camera = Rc::new(RefCell::new(Camera::new(
            Point3::new(3.5, 0., 0.),
            0.,
            PI,
            projection.clone(),
        )));

        let view_transform = camera.as_ref().borrow().transform();
        self.register_dynamic_object(camera.clone());
        self.main_camera = Some(camera);

        for i in 0..self.projection_uniforms.len() {
            self.projection_uniforms[i]
                .as_ref()
                .borrow_mut()
                .set_mat4(&projection);
            self.view_transform_uniforms[i]
                .as_ref()
                .borrow_mut()
                .set_mat4(&view_transform);
        }
    }

    pub fn init(&mut self) -> &mut Self {
        log::info!("initialising SDL...");
        self._init_sdl();
        log::info!("initialising OpenGL...");
        self._init_gl();
        log::info!("initialising shaders...");
        self._init_shaders();
        log::info!("initialising point of view...");
        self._init_point_of_view();
        log::info!("initialising objects...");
        self._init_objects();
        self
    }

    fn on_window_resize<'a>(
        projection_uniforms: impl Iterator<Item = &'a RefCell<Uniform>>,
        main_camera: &mut Camera,
        width: i32,
        height: i32,
    ) {
        unsafe {
            gl::Viewport(0, 0, width, height);
            gl_check!();
        }
        let aspect_ratio = width as f32 / height as f32;
        Self::update_perspective(projection_uniforms, main_camera, aspect_ratio);
    }

    fn update_perspective<'a>(
        projection_uniforms: impl Iterator<Item = &'a RefCell<Uniform>>,
        main_camera: &mut Camera,
        aspect_ratio: f32,
    ) {
        let projection = Matrix4::from(PerspectiveFov {
            aspect: aspect_ratio,
            far: definitions::DEFAULT_ZFAR,
            near: definitions::DEFAULT_ZNEAR,
            fovy: Rad(definitions::DEFAULT_FOV),
        });

        for u in projection_uniforms {
            u.borrow_mut().set_mat4(&projection);
        }

        main_camera.set_projection(projection);
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
                    keycode: Some(Keycode::B),
                    ..
                } => unsafe { BROKEN_RELATIVE_MOUSE_MODE = !BROKEN_RELATIVE_MOUSE_MODE },

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

                Event::Window {
                    win_event: WindowEvent::Resized(width, height),
                    ..
                } => Engine::on_window_resize(
                    self.projection_uniforms
                        .iter_mut()
                        .map(|x| /*dereferences ref then rc, then gets reference of RefCell*/ &**x),
                    &mut *self.main_camera.as_ref().unwrap().borrow_mut(),
                    width,
                    height,
                ),

                Event::MouseMotion {
                    xrel: x, yrel: y, ..
                } => unsafe {
                    if BROKEN_RELATIVE_MOUSE_MODE {
                        input.mouse_x_axis = (x - PREV_MOUSE_X) as f32;
                        input.mouse_y_axis = (y - PREV_MOUSE_Y) as f32;
                        PREV_MOUSE_X = x;
                        PREV_MOUSE_Y = y;
                    } else {
                        input.mouse_x_axis = x as f32;
                        input.mouse_y_axis = y as f32;
                    }
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
        let view_transform = self.main_camera.as_ref().unwrap().borrow().transform();

        for uniform in self.view_transform_uniforms.iter() {
            uniform.borrow_mut().set_mat4(&view_transform);
        }

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
