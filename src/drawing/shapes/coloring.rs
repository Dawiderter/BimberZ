use crate::{drawing::color::Color, math::{vector::{Vec2, vec2}, rect::{Rect, rect}}};

use super::shape::{Shape, Fragment};

#[derive(Debug, Clone, Copy)]
pub struct ColoredShape<A> {
    a: A,
    color: Color,
}

impl<A> ColoredShape<A> {
    pub fn new(a: A, color: Color) -> Self {
        Self { a, color }
    }
}

impl<A: Shape> Shape for ColoredShape<A> {
    fn frag(&self, v: crate::math::vector::Vec2<i32>) -> super::shape::Fragment {
        let child_frag = self.a.frag(v);
        Fragment::new(child_frag.dist, self.color)
    }

    fn bounding_box(&self) -> crate::math::rect::Rect<i32> {
        self.a.bounding_box()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StrokedShape<A> {
    pub a: A,
    pub width: i32,
}

impl<A> StrokedShape<A> {
    pub fn new(a: A, width: i32) -> Self {
        Self { a, width }
    }
}

impl<A: Shape> Shape for StrokedShape<A> {
    fn frag(&self, v: Vec2<i32>) -> Fragment {
        let child_frag = self.a.frag(v);
        Fragment::new(child_frag.dist.abs() - (self.width as f32) / 2.0, child_frag.color)
    }

    fn bounding_box(&self) -> Rect<i32> {
        let bb = self.a.bounding_box();
        rect(
            bb.top_left - vec2(self.width, self.width),
            bb.bot_right + vec2(self.width, self.width),
        )
    }
}

pub trait IntoColored where Self: Sized {
    fn colored(self, color: Color) -> ColoredShape<Self> {
        ColoredShape::new(self, color)
    }

    fn stroked(self, width: i32) -> StrokedShape<Self> {
        StrokedShape::new(self, width)
    }
}

impl<T : Shape> IntoColored for T {}