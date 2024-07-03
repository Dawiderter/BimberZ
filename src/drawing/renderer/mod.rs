use glam::Vec2;

use self::shape::{ConstOrBind, Shape};

pub mod shape;
pub mod bindings;
pub mod scene;
//pub mod program;
pub mod uniforms;

pub fn sphere(radius: impl Into<ConstOrBind<f32>>) -> Shape<'static> {
    Shape::new("sphere", vec![radius.into().into()])
}

pub fn torus(radius_and_thickness: impl Into<ConstOrBind<Vec2>>) -> Shape<'static> {
    Shape::new("torus", vec![radius_and_thickness.into().into()])
}