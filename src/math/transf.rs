#![allow(clippy::let_and_return)]

use super::vector::{Vec2, vec2};

/// Nie chce mi się implementować macierzy na ten moment, więc transformacje wyglądają w ten sposób
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub translation: Vec2<f32>,
    pub rotation: f32,
    pub scale: Vec2<f32>,
}

impl Transform {
    pub const ID: Self = Self { translation: vec2(0.0, 0.0), rotation: 0.0, scale: vec2(1.0,1.0) };

    pub fn mov_rot_scale(translation: Vec2<f32>, rotation: f32, scale: Vec2<f32>) -> Self {
        Self { translation, rotation, scale }
    }

    pub fn mov(translation: Vec2<f32>) -> Self {
        Self::mov_rot_scale(translation, 0.0, vec2(1.0, 1.0))
    }

    pub fn rot(rotation: f32) -> Self {
        Self::mov_rot_scale(vec2(0.0, 0.0), rotation, vec2(1.0, 1.0))
    }

    pub fn scale(scale: Vec2<f32>) -> Self {
        Self::mov_rot_scale(vec2(0.0, 0.0), 0.0, scale)
    }

    pub fn uscale(scale: f32) -> Self {
        Self::scale(vec2(scale, scale))
    }

    pub fn apply_to_pos(&self, pos: Vec2<f32>) -> Vec2<f32> {
        let scaled = vec2(self.scale.x * pos.x, self.scale.y * pos.y);
        let (s, c) = self.rotation.sin_cos();
        let rotated = vec2(c * scaled.x - s * scaled.y, s * scaled.x + c * scaled.y);
        let moved = rotated + self.translation;
        moved
    }

    pub fn apply_to_dir(&self, dir: Vec2<f32>) -> Vec2<f32> {
        let scaled = vec2(self.scale.x * dir.x, self.scale.y * dir.y);
        let (s, c) = self.rotation.sin_cos();
        let rotated = vec2(c * scaled.x - s * scaled.y, s * scaled.x + c * scaled.y);
        rotated
    }   

    pub fn apply_inv_to_pos(&self, pos: Vec2<f32>) -> Vec2<f32> {
        let moved = pos - self.translation;
        let (s, c) = (-self.rotation).sin_cos();
        let rotated = vec2(c * moved.x - s * moved.y, s * moved.x + c * moved.y);
        let scaled = vec2(rotated.x / self.scale.x, rotated.y / self.scale.y);
        scaled 
    }

    pub fn apply_inv_to_dir(&self, pos: Vec2<f32>) -> Vec2<f32> {
        let (s, c) = (-self.rotation).sin_cos();
        let rotated = vec2(c * pos.x - s * pos.y, s * pos.x + c * pos.y);
        let scaled = vec2(rotated.x / self.scale.x, rotated.y / self.scale.y);
        scaled 
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::*;

    #[test]
    fn apply_test() {
        let t_pos = vec2(1.0, 1.0);

        let transf = Transform::mov_rot_scale(vec2(0.0, 10.0), PI / 2.0, vec2(5.0, 1.0));

        let t_transf = transf.apply_to_pos(t_pos);

        dbg!(t_transf);

        let t_inv_transf = transf.apply_inv_to_pos(t_transf);

        dbg!(t_inv_transf);
    }
}