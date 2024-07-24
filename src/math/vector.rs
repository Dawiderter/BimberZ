use std::ops::{Add, Div, Mul, Neg, Sub};

pub type IVec2 = Vec2<i32>;
pub type UVec2 = Vec2<u32>;
pub type FVec2 = Vec2<f32>;

pub const fn vec2<T>(x: T, y: T) -> Vec2<T> {
    Vec2::new(x, y)
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::AnyBitPattern)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

unsafe impl bytemuck::NoUninit for Vec2<f32> {}

impl<T> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> Vec2<T> {
    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn len_sq(self) -> T {
        self.dot(self)
    }
}

impl Vec2<u32> {
    pub const ZERO: Self = vec2(0, 0);

    pub fn to_float(self) -> Vec2<f32> {
        self.into()
    }

    pub fn to_sign(self) -> Vec2<i32> {
        self.into()
    }
}

impl Vec2<i32> {
    pub const ZERO: Self = vec2(0, 0);

    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn to_float(self) -> Vec2<f32> {
        self.into()
    }
}

impl Vec2<f32> {
    pub const ZERO: Self = vec2(0.0, 0.0);

    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn len(self) -> f32 {
        self.len_sq().sqrt()
    }

    pub fn norm(self) -> Self {
        self / self.len()
    }

    pub fn to_int(self) -> Vec2<i32> {
        self.into()
    }
}

pub trait MaxMin {
    fn _max(self, rhs: Self) -> Self;
    fn _min(self, rhs: Self) -> Self;
    fn _clamp(self, min: Self, max: Self) -> Self;
}

impl MaxMin for u32 {
    fn _max(self, rhs: Self) -> Self {
        self.max(rhs)
    }

    fn _min(self, rhs: Self) -> Self {
        self.min(rhs)
    }

    fn _clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}

impl MaxMin for i32 {
    fn _max(self, rhs: Self) -> Self {
        self.max(rhs)
    }

    fn _min(self, rhs: Self) -> Self {
        self.min(rhs)
    }

    fn _clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}

impl MaxMin for f32 {
    fn _max(self, rhs: Self) -> Self {
        self.max(rhs)
    }

    fn _min(self, rhs: Self) -> Self {
        self.min(rhs)
    }

    fn _clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}

impl<T: MaxMin> Vec2<T> {
    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: self.x._max(rhs.x),
            y: self.y._max(rhs.y),
        }
    }

    pub fn min(self, rhs: Self) -> Self {
        Self {
            x: self.x._min(rhs.x),
            y: self.y._min(rhs.y),
        }
    }

    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self {
            x: self.x._clamp(min.x, max.x),
            y: self.y._clamp(min.y, max.y),
        }
    }
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Neg<Output = T>> Neg for Vec2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl From<Vec2<i32>> for Vec2<f32> {
    fn from(value: Vec2<i32>) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

impl From<Vec2<u32>> for Vec2<i32> {
    fn from(value: Vec2<u32>) -> Self {
        Self {
            x: value.x as i32,
            y: value.y as i32,
        }
    }
}

impl From<Vec2<u32>> for Vec2<f32> {
    fn from(value: Vec2<u32>) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

impl From<Vec2<f32>> for Vec2<i32> {
    fn from(value: Vec2<f32>) -> Self {
        Self {
            x: value.x.round() as i32,
            y: value.y.round() as i32,
        }
    }
}
