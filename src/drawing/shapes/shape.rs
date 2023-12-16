use crate::math::{vector::Vec2, rect::Rect};

pub trait Shape {
    fn dist(&self, v: Vec2<i32>) -> f32;
    fn bounding_box(&self) -> Rect<i32>;

    fn contains(&self, v: Vec2<i32>) -> bool {
        self.dist(v) < 0.5
    }
}