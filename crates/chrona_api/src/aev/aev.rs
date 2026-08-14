use std::collections::HashSet;

use winit::{event::ElementState, keyboard::{KeyCode, ModifiersState, PhysicalKey}};


pub struct CHAPI {
    pub delta_time: f32,
    
    pub keyboard_handler: KEYBOARD,
}

pub struct KEYBOARD {
    pub pressed_keys: HashSet<PhysicalKey>,
    pub modifiers: ModifiersState, 
}

impl KEYBOARD {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            modifiers: ModifiersState::default(),
        }
    }

    pub fn update_key(&mut self, physical_key: PhysicalKey, state: ElementState) {
        match state {
            ElementState::Pressed => { self.pressed_keys.insert(physical_key); }
            ElementState::Released => { self.pressed_keys.remove(&physical_key); }
        }
    }
    
    pub fn update_modifiers(&mut self, new_modifiers: ModifiersState) {
        self.modifiers = new_modifiers;
    }

    pub fn is_pressed(&self, key_code: KeyCode) -> bool {
        self.pressed_keys.contains(&PhysicalKey::Code(key_code))
    }
}

impl CHAPI {
    pub fn init() -> Self {

        Self {
            delta_time: 0.0,
            keyboard_handler: KEYBOARD::new()
        }
    }
}