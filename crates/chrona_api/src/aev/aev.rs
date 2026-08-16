use std::collections::HashSet;

use winit::{dpi::PhysicalPosition, event::ElementState, keyboard::{KeyCode, ModifiersState, PhysicalKey}};


pub struct CHAPI {
    pub delta_time: f32,
    
    pub keyboard_handler: KEYBOARD,
    pub mouse_handler: MOUSE
}

pub struct KEYBOARD {
    pub pressed_keys: HashSet<PhysicalKey>,
    pub modifiers: ModifiersState, 
}

pub struct MOUSE {
    pub grabbed: bool,

    pub x: f32,
    pub y: f32,
    pub delta_x: f32,
    pub delta_y: f32,

    pub accumulated_delta: (f64, f64),
}

impl MOUSE {
    pub fn new() -> Self {
        Self {
            grabbed: false,

            x: 0.0,
            y: 0.0,
            delta_x: 0.0,
            delta_y: 0.0,
            accumulated_delta: (0.0, 0.0),
        }
    }

    pub fn update_pos(&mut self, curpos: PhysicalPosition<f64>) {
        self.x = curpos.x as f32;
        self.y = curpos.y as f32;
    }

    pub fn update_moution(&mut self, delta: (f64, f64)) {
        if self.grabbed {
            self.accumulated_delta.0 += delta.0;
            self.accumulated_delta.1 += delta.1;
        }
    }

    pub fn update(&mut self) {
        if self.grabbed { 
            let current_delta = self.accumulated_delta;

            self.accumulated_delta = (0.0, 0.0);
            self.delta_x = current_delta.0 as f32;
            self.delta_y = current_delta.1 as f32;
        }
    }
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
            keyboard_handler: KEYBOARD::new(),
            mouse_handler: MOUSE::new(),
        }
    }
}