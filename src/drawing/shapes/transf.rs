use crate::math::{rect::rect, transf::Transform, vector::{vec2, Vec2}};

use super::shape::Shape;

#[derive(Debug, Clone, Copy)]
pub struct TransformedShape<A> {
    pub a: A,
    pub transform: Transform,
}

impl<A> TransformedShape<A> {
    pub fn new(a: A, transform: Transform) -> Self {
        Self { a, transform }
    }
}

impl<A: Shape> Shape for TransformedShape<A> {
    fn bounding_box(&self) -> crate::math::rect::Rect<i32> {
        let bb = self.a.bounding_box();
        let top_left = self.transform.apply_to_pos(bb.top_left.to_float()).to_int();
        let bot_right = self.transform.apply_to_pos(bb.bot_right.to_float()).to_int();
        let top_right = self.transform.apply_to_pos(vec2(bb.bot_right.x, bb.top_left.y).to_float()).to_int();
        let bot_left = self.transform.apply_to_pos(vec2(bb.top_left.x, bb.bot_right.y).to_float()).to_int();

        rect(
            top_left.min(bot_right).min(bot_left).min(top_right),
            top_left.max(bot_right).max(bot_left).max(top_right),
        )
    }

    fn frag(&self, v: Vec2<i32>) -> super::shape::Fragment {
        let d = self.transform.apply_inv_to_pos(v.to_float()).to_int();
        self.a.frag(d)
    }
}

pub trait IntoTransformed where Self: Sized {
    fn moved(self, translation: Vec2<f32>) -> TransformedShape<Self> {
        TransformedShape::new(self, Transform::mov(translation))
    }
    fn rotated(self, angle: f32) -> TransformedShape<Self> {
        TransformedShape::new(self, Transform::rot(angle))
    }
    fn scaled(self, scale: Vec2<f32>) -> TransformedShape<Self> {
        TransformedShape::new(self, Transform::scale(scale))
    }
}

impl<T: Shape> IntoTransformed for T {}