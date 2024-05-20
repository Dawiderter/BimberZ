use crate::{math::{vector::Vec2, rect::Rect}, drawing::color::Color};

pub struct Fragment {
    pub dist: f32,
    pub color: Color,
}

impl Fragment {
    pub fn new(dist: f32, color: Color) -> Self {
        Self { dist, color }
    }
    pub fn with_default_color(dist: f32) -> Self {
        Self::new(dist, Color::WHITE)
    }
}

pub trait Shape {
    fn frag(&self, v: Vec2<i32>) -> Fragment;
    fn bounding_box(&self) -> Rect<i32>;
    fn contains(&self, v: Vec2<i32>) -> bool {
        self.frag(v).dist <= 0.0
    }
}