use crate::math::{vector::{Vec2, vec2}, rect::{Rect, rect}};

use super::shape::{Shape, Fragment};

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub radius: i32,
}

impl Circle {
    pub fn new(radius: i32) -> Self {
        Self { radius }
    }
}

impl Shape for Circle {
    fn bounding_box(&self) -> Rect<i32> {
        let r = self.radius;
        rect(-vec2(r, r), vec2(r, r))
    }

    fn frag(&self, v: Vec2<i32>) -> super::shape::Fragment {
        let dist = v.to_float().len() - self.radius as f32;
        Fragment::with_default_color(dist)
    }
}
