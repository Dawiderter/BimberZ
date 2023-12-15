use super::color::Color;

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

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        self.buffer[(x + y * self.width) as usize]
    }
}
