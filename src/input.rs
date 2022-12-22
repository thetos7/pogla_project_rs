#[derive(Default, Debug)]
pub struct InputState {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub mouse_x_axis: f32,
    pub mouse_y_axis: f32,
    pub capture_cursor: bool,
    pub focused: bool,
}

static mut INSTANCE: Option<InputState> = None;

impl InputState {
    fn new() -> Self {
        InputState {
            focused: true,
            capture_cursor: true,
            ..Default::default()
        }
    }

    pub unsafe fn get() -> &'static Self {
        INSTANCE.get_or_insert_with(InputState::new)
    }

    pub unsafe fn get_mut() -> &'static mut Self {
        INSTANCE.get_or_insert_with(InputState::new)
    }
}
