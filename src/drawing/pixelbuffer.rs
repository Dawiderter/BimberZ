use crate::math::{rect::{Rect, rect}, vector::vec2};

use super::{color::Color, shapes::shape::Shape};

#[derive(Debug, Clone)]
pub struct PixelBuffer {
    buffer: Vec<Color>,
    width: u32,
    height: u32,
}

impl PixelBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let buffer = vec![Color::BLACK; width as usize * height as usize];
        Self {
            buffer,
            width,
            height,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.buffer.resize(width as usize * height as usize, Color::BLACK);
        self.width = width;
        self.height = height;
    }

    pub fn clear(&mut self, color: Color) {
        self.buffer.fill(color);
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn inner_buffer(&self) -> &[u8] {
        bytemuck::cast_slice(&self.buffer)
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.buffer[(x + y * self.width) as usize] = color;
    }

    pub fn frame_rect(&self) -> Rect<i32> {
        rect(vec2(0, 0), vec2(self.width as i32 - 1, self.height as i32 - 1))
    }

    pub fn draw_shape(&mut self, shape: impl Shape) {
        let bb = shape.bounding_box().intersection(self.frame_rect());

        for x in bb.top_left.x..=bb.bot_right.x {
            for y in bb.top_left.y..=bb.bot_right.y {
                let frag = shape.frag(vec2(x, y));
                if frag.dist <= 0.0 {
                    self.put_pixel(x as u32, y as u32, frag.color);
                }
            }
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        self.buffer[(x + y * self.width) as usize]
    }
}
