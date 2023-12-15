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

    pub fn draw_shape_filled(&mut self, shape: &impl Shape, color: Color) {
        let (top_left, bot_right) = shape.bounding_box();
        let top_left = (top_left.0.max(0) as u32, top_left.1.max(0) as u32);
        let bot_right = (bot_right.0.min(self.width() as i32 - 1) as u32, bot_right.1.min(self.height() as i32 - 1) as u32);

        for x in top_left.0..=bot_right.0 {
            for y in top_left.1..=bot_right.1 {
                if shape.dist(x as i32, y as i32) < 0.5 {
                    self.put_pixel(x, y, color);
                }
            }
        }
    }

    pub fn draw_shape_stroke(&mut self, shape: &impl Shape, color: Color) {
        let (top_left, bot_right) = shape.bounding_box();
        let top_left = (top_left.0.max(0) as u32, top_left.1.max(0) as u32);
        let bot_right = (bot_right.0.min(self.width() as i32 - 1) as u32, bot_right.1.min(self.height() as i32 - 1) as u32);

        for x in top_left.0..=bot_right.0 {
            for y in top_left.1..=bot_right.1 {
                let d = shape.dist(x as i32, y as i32); 
                if (-0.5..0.5).contains(&d) {
                    self.put_pixel(x, y, color);
                }
            }
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        self.buffer[(x + y * self.width) as usize]
    }
}
