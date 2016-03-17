use std::sync::{Arc, RwLock};
use std::collections::{HashMap};

use input::{Keyboard, Mouse, Display, Button};
use logic::{Being, BeingType};
use utils::{KeyCode, MouseButton};
use math::{Vec2};

pub struct World<T: BeingType> {
    keyboard: Arc<RwLock<Keyboard>>,
    mouse: Arc<RwLock<Mouse>>,
    display: Arc<RwLock<Display>>,
    beings: Arc<RwLock<HashMap<u64, Arc<RwLock<Box<Being<T>>>>>>>,
}

impl<T: BeingType> World<T> {
    pub fn new(keyboard: Arc<RwLock<Keyboard>>, mouse: Arc<RwLock<Mouse>>, display: Arc<RwLock<Display>>) -> World<T> {
        World {
            keyboard: keyboard,
            mouse: mouse,
            display: display,
            beings: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get_beings(&self) -> Arc<RwLock<HashMap<u64, Arc<RwLock<Box<Being<T>>>>>>> {
        self.beings.clone()
    }

    pub fn get_key(&self, key_code: KeyCode) -> Button {
        self.keyboard.read().expect("Unable to Read Keyboard in Get Key in World").get_key(key_code)
    }

    pub fn get_mouse_button(&self, mouse_button: MouseButton) -> Button {
        self.mouse.read().expect("Unable to Read Mouse in Get Mouse Button in World").get_button(mouse_button)
    }

    pub fn get_mouse_position(&self) -> Vec2 {
        self.mouse.read().expect("Unable to Read Mouse in Get Mouse Position in World").get_mouse_position()
    }

    pub fn get_resolution(&self) -> Vec2 {
        self.display.read().expect("Unable to Read Display in Get Resolution in World").get_resolution()
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.display.read().expect("Unable to Read Display in Get Aspect Ratio in World").get_aspect_ratio()
    }
}
