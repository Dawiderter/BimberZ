use crate::math::{rect::rect, transf::Transform, vector::vec2};

use super::shape::Shape;

#[derive(Debug, Clone, Copy)]
pub struct TransformedShape<A> {
    pub a: A,
    pub t: Transform,
}

impl<A: Shape> Shape for TransformedShape<A> {
    fn dist(&self, v: crate::math::vector::Vec2<i32>) -> f32 {
        let d = self.t.apply_inv_to_pos(v.to_float()).to_int();
        self.a.dist(d)
    }

    fn bounding_box(&self) -> crate::math::rect::Rect<i32> {
        let bb = self.a.bounding_box();
        let top_left = self.t.apply_to_pos(bb.top_left.to_float()).to_int();
        let bot_right = self.t.apply_to_pos(bb.bot_right.to_float()).to_int();
        let top_right = self.t.apply_to_pos(vec2(bb.bot_right.x, bb.top_left.y).to_float()).to_int();
        let bot_left = self.t.apply_to_pos(vec2(bb.top_left.x, bb.bot_right.y).to_float()).to_int();

        rect(
            top_left.min(bot_right).min(bot_left).min(top_right),
            top_left.max(bot_right).max(bot_left).max(top_right),
        )
    }
}
