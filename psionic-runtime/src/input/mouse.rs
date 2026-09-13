use winit::event::MouseButton;

pub struct MouseButtonMapping {
    pub name: String,
    pub mouse_button: MouseButton,
}

pub struct MouseButtonState {
    pub mouse_button: MouseButton,
    pub is_down: bool,
    pub down_this_frame: bool,
    pub up_this_frame: bool,
}

pub struct MousePositionState {
    pub position: (f32, f32),
    pub outside_window: bool,
}


impl MouseButtonState {
    pub fn new(mouse_button: &MouseButton) -> Self {
        Self {
            mouse_button: mouse_button.clone(),
            is_down: false,
            down_this_frame: false,
            up_this_frame: false,
        }
    }
}