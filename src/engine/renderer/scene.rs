use std::borrow::Cow;

use glam::{Vec2, Vec3};

use super::uniforms::Uniform;

pub fn sphere(radius: impl Into<ConstOrUniform<f32>>) -> Shape<'static> {
    Shape::new("sphere", vec![radius.into().into()])
}

pub fn sdbox(half_diag: impl Into<ConstOrUniform<Vec3>>) -> Shape<'static> {
    Shape::new("box", vec![half_diag.into().into()])
}

pub struct Scene {
    pub shape: Shape<'static>,
    pub has_changed: bool,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            shape: sphere(0.0),
            has_changed: true,
        }
    }

    pub fn to_wgsl(&self) -> String {
        let mut res = String::new();

        res.push_str(&self.shape.to_wgsl_begin());
        res.push('p');
        res.push_str(&self.shape.to_wgsl_end());

        res
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

pub enum ConstOrUniform<T> {
    Const(T),
    Uniform(Uniform<T>),
}

#[derive(Debug, Clone)]
pub struct ShapeParameter(String);

#[derive(Debug)]
pub struct Shape<'a> {
    pub func: Cow<'a, str>,
    pub parameters: Vec<ShapeParameter>,
}

impl<'a> Shape<'a> {
    pub fn new(func: impl Into<Cow<'a, str>>, parameters: Vec<ShapeParameter>) -> Self {
        Self {
            func: func.into(),
            parameters,
        }
    }
    pub fn to_wgsl_begin(&self) -> String {
        format!("{}(", self.func)
    }
    pub fn to_wgsl_end(&self) -> String {
        let mut s = String::new();
        for param in &self.parameters {
            s.push(',');
            s.push_str(&param.0);
        }
        s.push(')');
        s
    }
}

impl From<f32> for ConstOrUniform<f32> {
    fn from(value: f32) -> Self {
        Self::Const(value)
    }
}

impl From<Vec2> for ConstOrUniform<Vec2> {
    fn from(value: Vec2) -> Self {
        Self::Const(value)
    }
}

impl From<Vec3> for ConstOrUniform<Vec3> {
    fn from(value: Vec3) -> Self {
        Self::Const(value)
    }
}

impl<T> From<Uniform<T>> for ConstOrUniform<T> {
    fn from(value: Uniform<T>) -> Self {
        Self::Uniform(value)
    }
}

impl<T: Into<ShapeParameter>> From<ConstOrUniform<T>> for ShapeParameter {
    fn from(value: ConstOrUniform<T>) -> Self {
        match value {
            ConstOrUniform::Const(c) => c.into(),
            ConstOrUniform::Uniform(b) => b.into(),
        }
    }
}

impl From<f32> for ShapeParameter {
    fn from(value: f32) -> Self {
        ShapeParameter(format!("{value:?}"))
    }
}

impl From<Vec2> for ShapeParameter {
    fn from(value: Vec2) -> Self {
        ShapeParameter(format!("vec2f({:?},{:?})", value.x, value.y))
    }
}

impl From<Vec3> for ShapeParameter {
    fn from(value: Vec3) -> Self {
        ShapeParameter(format!("vec2f({:?},{:?})", value.x, value.y))
    }
}

impl<T> From<Uniform<T>> for ShapeParameter {
    fn from(value: Uniform<T>) -> Self {
        ShapeParameter(format!("data.s{}", value.idx()))
    }
}

impl From<String> for ShapeParameter {
    fn from(value: String) -> Self {
        ShapeParameter(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let shape = Shape {
            func: "circle".into(),
            parameters: vec![1.0.into()],
        };
        eprintln!("{}p_t{}", shape.to_wgsl_begin(), shape.to_wgsl_end());
    }
}
