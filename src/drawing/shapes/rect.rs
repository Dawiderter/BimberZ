use crate::math::{rect::Rect, vector::{Vec2, vec2}};

use super::shape::Shape;

#[derive(Debug, Clone, Copy)]
pub struct RectShape {
    pub rect: Rect<i32>,
}

impl Shape for RectShape {
    fn dist(&self, v: Vec2<i32>) -> f32 {
        let center = (self.rect.top_left + self.rect.bot_right).to_float() / 2.0;
        let corner = self.rect.bot_right.to_float() - center;
        let diff = (v.to_float() - center).abs() - corner;
        diff.max(vec2(0.0, 0.0)).len() + diff.x.max(diff.y).min(0.0)
    }

    fn bounding_box(&self) -> Rect<i32> {
        self.rect
    }
}