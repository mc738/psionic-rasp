use winit::keyboard::KeyCode;

pub struct KeyboardKeyMapping {
    pub name: String,
    pub key_code: KeyCode,
}

pub struct KeyboardKeyState {
    pub key_code: KeyCode,
    pub is_down: bool,
    pub down_this_frame: bool,
    pub up_this_frame: bool,
}

impl KeyboardKeyState {
    pub fn new(key_code: &KeyCode) -> Self {
        Self {
            key_code: key_code.clone(),
            is_down: false,
            down_this_frame: false,
            up_this_frame: false,
        }
    }
}