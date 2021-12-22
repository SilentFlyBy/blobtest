use std::{collections::HashSet, fmt};

use sdl2::{keyboard::Keycode, EventPump};

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum InputCode {
    Right,
    Left,
    Up,
    Down,
}

impl InputCode {
    pub fn from_keycode(keycode: Keycode) -> Option<InputCode> {
        match keycode {
            Keycode::Up => Some(InputCode::Up),
            Keycode::Down => Some(InputCode::Down),
            Keycode::Right => Some(InputCode::Right),
            Keycode::Left => Some(InputCode::Left),
            _ => None,
        }
    }
}

impl fmt::Display for InputCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

pub struct InputManager {
    inputs: HashSet<InputCode>,
}

impl InputManager {
    pub fn new() -> InputManager {
        InputManager {
            inputs: HashSet::new(),
        }
    }

    pub fn process_events(&mut self, event_pump: &mut EventPump) {
        self.inputs.clear();
        event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .for_each(|k| {
                let input_code = InputCode::from_keycode(k);
                if input_code.is_some() {
                    self.inputs.insert(input_code.unwrap());
                }
            });
    }

    pub fn right(&self) -> bool {
        return self.inputs.contains(&InputCode::Right);
    }

    pub fn left(&self) -> bool {
        return self.inputs.contains(&InputCode::Left);
    }
}
