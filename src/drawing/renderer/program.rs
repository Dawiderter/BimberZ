use super::{bindings::BindingsBuffer, scene::Scene};

pub struct Program {
    pub bindings: BindingsBuffer,
    pub scene: Scene<'static>
}