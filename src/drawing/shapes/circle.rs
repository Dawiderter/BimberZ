use crate::math::{vector::{Vec2, vec2}, rect::{Rect, rect}};

use super::shape::Shape;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Vec2<i32>,
    pub radius: i32,
}

impl Shape for Circle {
    fn dist(&self, v: Vec2<i32>) -> f32 {
        (self.center - v).to_float().len() - self.radius as f32
    }

    fn bounding_box(&self) -> Rect<i32> {
        let r = self.radius;
        rect(self.center - vec2(r, r), self.center + vec2(r, r))
    }
}
