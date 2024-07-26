use std::fmt::Display;

use glam::{Quat, Vec2, Vec3};

use super::uniforms::Uniform;

pub fn sdsphere(radius: impl Into<ConstOrUniform<f32>>) -> SceneNode {
    SceneNode::Shape(Shape(WgslCall::new(
        "sdsphere".into(),
        vec![radius.into().into()],
    )))
}

pub fn sdbox(half_diag: impl Into<ConstOrUniform<Vec3>>) -> SceneNode {
    SceneNode::Shape(Shape(WgslCall::new(
        "sdbox".into(),
        vec![half_diag.into().into()],
    )))
}

pub struct Scene {
    pub shape: SceneNode,
    pub has_changed: bool,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            shape: sdsphere(0.0),
            has_changed: true,
        }
    }

    pub fn to_wgsl(&self) -> String {
        format!("{}", SceneWgsl { scene: self })
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum SceneNode {
    Shape(Shape),
    Operator(Operator),
}

#[derive(Debug)]
pub struct Shape(WgslCall);

#[derive(Debug)]
pub struct Operator {
    nodes: Vec<SceneNode>,
    call: WgslCall,
    modifier: Option<PointModifier>,
}

impl SceneNode {
    pub fn translated(self, t: impl Into<ConstOrUniform<Vec3>>) -> Self {
        Self::Operator(Operator {
            nodes: vec![self],
            call: WgslCall::new("optranslate".into(), vec![]),
            modifier: Some(PointModifier(WgslCall {
                func: "opinvtranslate".into(),
                parameters: vec![t.into().into()],
            })),
        })
    }
    pub fn rotated(self, q: impl Into<ConstOrUniform<Quat>>) -> Self {
        Self::Operator(Operator {
            nodes: vec![self],
            call: WgslCall::new("oprotate".into(), vec![]),
            modifier: Some(PointModifier(WgslCall {
                func: "opinvrotate".into(),
                parameters: vec![q.into().into()],
            })),
        })
    }
    pub fn rounded(self, r: impl Into<ConstOrUniform<f32>>) -> Self {
        Self::Operator(Operator {
            nodes: vec![self],
            call: WgslCall::new("opround".into(), vec![r.into().into()]),
            modifier: None,
        })
    }
    pub fn union(self, other: Self) -> Self {
        Self::Operator(Operator {
            nodes: vec![self, other],
            call: WgslCall::new("opunion".into(), vec![]),
            modifier: None,
        })
    }
    pub fn smooth_union(self, other: Self, k: impl Into<ConstOrUniform<f32>>) -> Self {
        Self::Operator(Operator {
            nodes: vec![self, other],
            call: WgslCall::new("opsmoothunion".into(), vec![k.into().into()]),
            modifier: None,
        })
    }
}

#[derive(Debug, Clone)]
struct PointModifier(WgslCall);

#[derive(Debug, Clone)]
struct Parameter(String);

#[derive(Debug, Clone)]
struct WgslCall {
    func: String,
    parameters: Vec<Parameter>,
}

struct SceneWgsl<'scene> {
    scene: &'scene Scene,
}

struct WgslPrologue<'call> {
    call: &'call WgslCall,
}

struct WgslEpilogue<'shape> {
    call: &'shape WgslCall,
}

impl Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'scene> Display for SceneWgsl<'scene> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut modifiers = Vec::new();
        self.scene.shape.fmt_wgsl(f, &mut modifiers)
    }
}

impl SceneNode {
    fn fmt_wgsl(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        modifiers: &mut Vec<PointModifier>,
    ) -> std::fmt::Result {
        match self {
            SceneNode::Shape(shape) => {
                write!(f, "{}", shape.0.prologue())?;
                for modifier in modifiers.iter().rev() {
                    write!(f, "{}", modifier.0.prologue())?;
                }
                write!(f, "p")?;
                for modifier in modifiers.iter() {
                    write!(f, ",{}", modifier.0.epilogue())?;
                }
                write!(f, ",{}", shape.0.epilogue())?
            }
            SceneNode::Operator(operator) => {
                if let Some(modifier) = &operator.modifier {
                    modifiers.push(modifier.clone())
                }
                write!(f, "{}", operator.call.prologue())?;
                for node in &operator.nodes {
                    node.fmt_wgsl(f, modifiers)?;
                    write!(f, ",")?;
                }
                write!(f, "{}", operator.call.epilogue())?;
                if operator.modifier.is_some() {
                    modifiers.pop();
                }
            }
        }
        Ok(())
    }
}

impl WgslCall {
    fn new(func: String, parameters: Vec<Parameter>) -> Self {
        Self { func, parameters }
    }
    fn prologue(&self) -> WgslPrologue {
        WgslPrologue { call: self }
    }
    fn epilogue(&self) -> WgslEpilogue {
        WgslEpilogue { call: self }
    }
}

impl<'shape> Display for WgslPrologue<'shape> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(", self.call.func)
    }
}

impl<'shape> Display for WgslEpilogue<'shape> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for param in &self.call.parameters {
            write!(f, "{},", param)?;
        }
        write!(f, ")")
    }
}

pub enum ConstOrUniform<T> {
    Const(T),
    Uniform(Uniform<T>),
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

impl<T: Into<Parameter>> From<ConstOrUniform<T>> for Parameter {
    fn from(value: ConstOrUniform<T>) -> Self {
        match value {
            ConstOrUniform::Const(c) => c.into(),
            ConstOrUniform::Uniform(b) => b.into(),
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
        Parameter(format!("vec3f({:?},{:?},{:?})", value.x, value.y, value.z))
    }
}

impl From<Quat> for Parameter {
    fn from(value: Quat) -> Self {
        Parameter(format!(
            "vec4f({:?},{:?},{:?},{:?})",
            value.x, value.y, value.z, value.w
        ))
    }
}

impl<T> From<Uniform<T>> for Parameter {
    fn from(value: Uniform<T>) -> Self {
        Parameter(format!("data.s{}", value.idx()))
    }
}

impl From<String> for Parameter {
    fn from(value: String) -> Self {
        Parameter(value)
    }
}

#[cfg(test)]
mod tests {
    use glam::vec3;

    use super::*;

    #[test]
    fn sphere_test() {
        let node = sdsphere(5.0).translated(vec3(0.0, -1.0, 5.0));

        let scene = Scene {
            shape: node,
            has_changed: false,
        };

        println!("{}", scene.to_wgsl());
    }
}
