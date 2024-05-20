use delegate::delegate;

use super::{pixelbuffer::PixelBuffer, input::{Input, self}, color::Color, shapes::shape::Shape};

pub struct Frame {
    pub(crate) buffer: PixelBuffer,
    pub(crate) input: Input,
}

impl Frame {
    pub fn new(width: u32, height: u32) -> Self {
        Self { buffer: PixelBuffer::new(width, height), input: Input::init() }
    }
}

impl Frame {
    delegate! {
        to self.buffer {
            pub fn width(&self) -> u32;
            pub fn height(&self) -> u32;
            pub fn put_pixel(&mut self, x: u32, y: u32, color: Color);
            pub fn draw_shape(&mut self, shape: impl Shape);
            pub fn clear(&mut self, color: Color);
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