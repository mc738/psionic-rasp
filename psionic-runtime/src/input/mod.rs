
use std::collections::HashMap;
use winit::keyboard::KeyCode;

pub struct InputMap {
    keyboard_key_mappings: Vec<KeyboardKeyMapping>

}

pub struct KeyboardKeyMapping {
    name: String,
    key_code: KeyCode,
}

pub struct  InputManager {
    keyboard_keys_state: HashMap<KeyCode, KeyboardKeyState>
}


pub struct KeyboardKeyState {
    key_code: KeyCode,
    is_down: bool,
    down_this_frame: bool,
    up_this_frame: bool,
}

impl InputManager {

    pub fn new() -> InputManager {
        InputManager {
            keyboard_keys_state: Default::default(),
        }
    }

    pub fn load_input_map(&mut self, input_map: &InputMap) {
        self.keyboard_keys_state.clear();

        for keyboard_key_mapping in input_map.keyboard_key_mappings.iter() {
            self.keyboard_keys_state.insert(keyboard_key_mapping.key_code, KeyboardKeyState::new(&keyboard_key_mapping.key_code));
        }
    }

    pub fn update_keyboard_key_state(&mut self, key_code: &KeyCode, is_down: bool) {
        match self.keyboard_keys_state.get_mut(&key_code) {
            None => {
                // If the key is not mapped, ignore it.
            }
            Some(state) => {
                state.is_down = is_down;
                if is_down {
                    state.down_this_frame = true;
                }
                else {
                    state.up_this_frame = true;
                }
            }
        }
    }


    /// Resets all the up_this_frame and down_this_frame states.
    /// This is designed to be called at the end of the runtime loop.
    pub fn reset_frame_states(&mut self) {
        for keyboard_key_mapping in self.keyboard_keys_state.values_mut() {
            keyboard_key_mapping.up_this_frame = false;
            keyboard_key_mapping.down_this_frame = false;
        }
    }
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