use super::vector::{Vec2, MaxMin};

pub fn rect<T>(top_left: Vec2<T>, bot_right: Vec2<T>) -> Rect<T> {
    Rect::new(top_left, bot_right)
}

#[derive(Debug, Clone, Copy)]
pub struct Rect<T> {
    pub top_left: Vec2<T>,
    pub bot_right: Vec2<T>,
}

impl<T> Rect<T> {
    pub fn new(top_left: Vec2<T>, bot_right: Vec2<T>) -> Self {
        Self { top_left, bot_right }
    }
}

impl<T : MaxMin> Rect<T> {
    pub fn combine(self, rhs: Self) -> Self {
        Self { top_left: self.top_left.min(rhs.top_left), bot_right: self.bot_right.max(rhs.bot_right) }
    }

    pub fn intersection(self, rhs: Self) -> Self {
        Self { top_left: self.top_left.max(rhs.top_left), bot_right: self.bot_right.min(rhs.bot_right) }
    }
}