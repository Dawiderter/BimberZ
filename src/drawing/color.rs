
#[derive(Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g : u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK : Self = Color::new(0, 0, 0, 255);
    pub const WHITE : Self = Color::new(255, 255, 255, 255);

    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b , a}
    }
}