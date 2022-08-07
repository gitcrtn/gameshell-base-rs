use std::collections::HashSet;
use sdl2::keyboard::Keycode;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Keys {
    Up,
    Down,
    Left,
    Right,
    X,
    Y,
    A,
    B,
    Select,
    Start,
}

mod keys_context {
    use std::collections::HashMap;
    use lazy_static::lazy_static;
    use sdl2::keyboard::Keycode;
    use input::Keys;
    use crate::input;

    lazy_static! {
        pub static ref KEYS_DISPLAY: HashMap<Keys, String> = [
            (Keys::Up,          "U".to_string()),
            (Keys::Down,        "D".to_string()),
            (Keys::Left,        "L".to_string()),
            (Keys::Right,       "R".to_string()),
            (Keys::X,           "X".to_string()),
            (Keys::Y,           "Y".to_string()),
            (Keys::A,           "A".to_string()),
            (Keys::B,           "B".to_string()),
            (Keys::Select,      "[]".to_string()),
            (Keys::Start,       "[>".to_string()),
        ].into_iter().collect();

        pub static ref KEYS_MAP: HashMap<Keycode, Keys> = [
            (Keycode::Up,       Keys::Up),
            (Keycode::Down,     Keys::Down),
            (Keycode::Left,     Keys::Left),
            (Keycode::Right,    Keys::Right),
            (Keycode::U,        Keys::X),
            (Keycode::I,        Keys::Y),
            (Keycode::J,        Keys::A),
            (Keycode::K,        Keys::B),
            (Keycode::Space,    Keys::Select),
            (Keycode::Return,   Keys::Start),
        ].into_iter().collect();
    }
}

pub struct InputContext {
    keys_down: HashSet<Keycode>,
    keys_up: HashSet<Keycode>,
    keys_pressed: HashSet<Keycode>,
    keys_pulled: HashSet<Keycode>,
}

impl InputContext {
    pub fn new() -> Self {
        let ref keys_map = keys_context::KEYS_MAP;
        InputContext {
            keys_down: HashSet::new(),
            keys_up: HashSet::from_iter(keys_map.keys().cloned()),
            keys_pressed: HashSet::new(),
            keys_pulled: HashSet::new(),
        }
    }

    pub fn reset_frame(&mut self) {
        self.keys_pulled.clear();
        self.keys_pressed.clear();
    }

    pub fn on_key_down(&mut self, keycode: Keycode) {
        let ref keys_map = keys_context::KEYS_MAP;
        if !keys_map.contains_key(&keycode) {
            return;
        }
        if !self.keys_down.contains(&keycode) && self.keys_up.contains(&keycode) {
            self.keys_pressed.insert(keycode.clone());
        }
        self.keys_down.insert(keycode.clone());
        self.keys_up.remove(&keycode);
    }

    pub fn on_key_up(&mut self, keycode: Keycode) {
        let ref keys_map = keys_context::KEYS_MAP;
        if !keys_map.contains_key(&keycode) {
            return;
        }
        if self.keys_down.contains(&keycode) && !self.keys_up.contains(&keycode) {
            self.keys_pulled.insert(keycode.clone());
        }
        self.keys_up.insert(keycode.clone());
        self.keys_down.remove(&keycode);
    }

    pub fn get_keys_pressed(&mut self) -> Vec<Keys> {
        let ref keys_map = keys_context::KEYS_MAP;
        self.keys_pressed
            .iter()
            .map(|key| keys_map.get(key).unwrap().clone())
            .collect()
    }

    pub fn get_keys_pulled(&mut self) -> Vec<Keys> {
        let ref keys_map = keys_context::KEYS_MAP;
        self.keys_pulled
            .iter()
            .map(|key| keys_map.get(key).unwrap().clone())
            .collect()
    }

    pub fn get_keys_downed(&mut self) -> Vec<Keys> {
        let ref keys_map = keys_context::KEYS_MAP;
        self.keys_down
            .iter()
            .map(|key| keys_map.get(key).unwrap().clone())
            .collect()
    }
}

pub(crate) fn initialize_input() -> InputContext {
    InputContext::new()
}

pub fn get_keys_text(keys: &Vec<Keys>) -> String {
    let ref keys_display = keys_context::KEYS_DISPLAY;
    keys
        .iter()
        .map(|key| keys_display.get(key).unwrap().clone())
        .collect::<Vec<String>>()
        .join(",")
}