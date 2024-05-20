use crate::math::{rect::{Rect, rect}, vector::{Vec2, vec2}};

use super::shape::{Shape, Fragment};

#[derive(Debug, Clone, Copy)]
pub struct RectShape {
    pub size: Vec2<u32>
}

impl RectShape {
    pub fn new(size: Vec2<u32>) -> Self {
        Self { size }
    }
}

impl Shape for RectShape {
    fn bounding_box(&self) -> Rect<i32> {
        rect(-self.size.to_sign() / 2 - vec2(1, 1), self.size.to_sign() / 2 + vec2(1, 1))
    }

    fn frag(&self, v: Vec2<i32>) -> super::shape::Fragment {  
        let corner = self.size.to_float() / 2.0;
        let diff = v.to_float().abs() - corner;
        let dist = diff.max(vec2(0.0, 0.0)).len() + diff.x.max(diff.y).min(0.0);
        Fragment::with_default_color(dist)
    }
}