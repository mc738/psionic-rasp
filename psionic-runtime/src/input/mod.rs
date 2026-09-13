pub mod keyboard;
pub mod mouse;

pub use crate::input::keyboard::{KeyboardKeyMapping, KeyboardKeyState};
pub use crate::input::mouse::{MouseButtonMapping, MouseButtonState};
use std::collections::HashMap;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;
use crate::input::mouse::MousePositionState;

pub struct InputMap {
    pub keyboard_key_mappings: Vec<KeyboardKeyMapping>,
    pub mouse_button_mappings: Vec<MouseButtonMapping>,
}

pub struct InputManager {
    keyboard_keys_state: HashMap<KeyCode, KeyboardKeyState>,
    mouse_buttons_state: HashMap<MouseButton, MouseButtonState>,
    mouse_position_state: MousePositionState,
}

impl InputManager {
    pub fn new() -> InputManager {
        InputManager {
            keyboard_keys_state: Default::default(),
            mouse_buttons_state: Default::default(),
            mouse_position_state: MousePositionState {
                position: (0.0, 0.0),
                outside_window: false,
            },
        }
    }

    pub fn get_keyboard_key_state(&self, key: KeyCode) -> Option<&KeyboardKeyState> {
        self.keyboard_keys_state.get(&key)
    }

    pub fn get_mouse_button_state(&self, mouse_button: MouseButton) -> Option<&MouseButtonState> {
        self.mouse_buttons_state.get(&mouse_button)
    }

    pub fn get_mouse_position_state(&self) -> &MousePositionState {
        &self.mouse_position_state
    }

    pub fn load_input_map(&mut self, input_map: &InputMap) {
        self.keyboard_keys_state.clear();

        for keyboard_key_mapping in input_map.keyboard_key_mappings.iter() {
            self.keyboard_keys_state.insert(
                keyboard_key_mapping.key_code,
                KeyboardKeyState::new(&keyboard_key_mapping.key_code),
            );
        }

        for mouse_button_mapping in input_map.mouse_button_mappings.iter() {
            self.mouse_buttons_state.insert(
                mouse_button_mapping.mouse_button,
                MouseButtonState::new(&mouse_button_mapping.mouse_button),
            );
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
                } else {
                    state.up_this_frame = true;
                }
            }
        }
    }

    pub fn update_mouse_button_state(&mut self, mouse_button: MouseButton, is_down: bool) {
        match self.mouse_buttons_state.get_mut(&mouse_button) {
            None => {
                // If the key is not mapped, ignore it.
            }
            Some(state) => {
                state.is_down = is_down;
                if is_down {
                    state.down_this_frame = true;
                } else {
                    state.up_this_frame = true;
                }
            }
        }
    }

    pub fn update_mouse_position(&mut self, position: (f32, f32)) {
        self.mouse_position_state.position = position;
    }

    pub fn update_mouse_outside_window(&mut self, outside_window: bool) {
        self.mouse_position_state.outside_window = outside_window;
    }

    /// Resets all the up_this_frame and down_this_frame states.
    /// This is designed to be called at the end of the runtime loop.
    pub fn reset_frame_states(&mut self) {
        for keyboard_key_state in self.keyboard_keys_state.values_mut() {
            keyboard_key_state.up_this_frame = false;
            keyboard_key_state.down_this_frame = false;
        }

        for mouse_button_state in self.mouse_buttons_state.values_mut() {
            mouse_button_state.up_this_frame = false;
            mouse_button_state.down_this_frame = false;
        }
    }
}
