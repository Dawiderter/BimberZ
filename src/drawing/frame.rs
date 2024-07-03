use delegate::delegate;

use super::{renderer::bindings::{Bind, Bindings}, color::Color, input::{self, Input}};

pub struct Frame {
    pub bindings: Bindings,
    pub(crate) input: Input,
}

impl Frame {
    pub fn new(width: u32, height: u32) -> Self {
        Self { input: Input::init(), bindings: Bindings::new() }
    }
}

impl Frame {
    delegate! {
        to self.bindings {
            pub fn bind<T>(&mut self, slot: usize) -> Bind<T>;
        }
    }
}

impl Frame {
    delegate! {
        to self.input {
            pub fn is_key_pressed(&self, code : input::Key) -> bool;
            pub fn is_mouse_pressed(&self, button: input::Mouse) -> bool;
            pub fn on_key_tap(&self, code : input::Key) -> bool;
            pub fn on_mouse_tap(&self, button: input::Mouse);
            pub fn mouse_position(&self) -> (f64,f64);
            pub fn mouse_motion(&self) -> (f64,f64);
        }
    }
}