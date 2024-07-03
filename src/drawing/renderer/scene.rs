use super::shape::Shape;

const TEMPLATE : &str = include_str!("./template.wgsl");

pub struct Scene<'a> {
    pub shape: Shape<'a>
}

impl<'a> Scene<'a> {
    pub fn to_wgsl(&self) -> String {
        let scene = format!("{}p{}", self.shape.to_wgsl_begin(), self.shape.to_wgsl_end());
        TEMPLATE.replace("//{{SCENE}}", &scene)
    }
}