use std::borrow::Cow;

use glam::{Vec2, Vec3};

use super::bindings::Bind;

pub enum ConstOrBind<T> {
    Const(T),
    Bind(Bind<T>),
}

#[derive(Debug, Clone)]
pub struct Parameter(String);

#[derive(Debug)]
pub struct Shape<'a> {
    pub func: Cow<'a, str>,
    pub parameters: Vec<Parameter>,
}

impl<'a> Shape<'a> {
    pub fn new(func: impl Into<Cow<'a, str>>, parameters: Vec<Parameter>) -> Self {
        Self { func: func.into(), parameters }
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

impl From<f32> for ConstOrBind<f32> {
    fn from(value: f32) -> Self {
        Self::Const(value)
    }
}

impl From<Vec2> for ConstOrBind<Vec2> {
    fn from(value: Vec2) -> Self {
        Self::Const(value)
    }
}

impl From<Vec3> for ConstOrBind<Vec3> {
    fn from(value: Vec3) -> Self {
        Self::Const(value)
    }
}

impl<T> From<Bind<T>> for ConstOrBind<T> {
    fn from(value: Bind<T>) -> Self {
        Self::Bind(value)
    }
}

impl<T : Into<Parameter>> From<ConstOrBind<T>> for Parameter {
    fn from(value: ConstOrBind<T>) -> Self {
        match value {
            ConstOrBind::Const(c) => c.into(),
            ConstOrBind::Bind(b) => b.into(),
        }
    }
}

impl From<f32> for Parameter {
    fn from(value: f32) -> Self {
        Parameter(format!("{value:?}"))
    }
}

impl From<Vec2> for Parameter {
    fn from(value: Vec2) -> Self {
        Parameter(format!("vec2f({:?},{:?})", value.x, value.y))
    }
}

impl From<Vec3> for Parameter {
    fn from(value: Vec3) -> Self {
        Parameter(format!("vec2f({:?},{:?})", value.x, value.y))
    }
}

impl<T> From<Bind<T>> for Parameter {
    fn from(value: Bind<T>) -> Self {
        Parameter(format!("data.s{}", value.slot))
    }
}

impl From<String> for Parameter {
    fn from(value: String) -> Self {
        Parameter(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let shape = Shape { func: "circle".into(), parameters: vec![1.0.into()] };
        eprintln!("{}p_t{}", shape.to_wgsl_begin(), shape.to_wgsl_end());
    }
}