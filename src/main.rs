use std::f32::consts::PI;

use bimberz::{
    drawing::{
        color::Color,
        shapes::{
            circle::Circle,
            coloring::IntoColored,
            composite::{ShapeDiff, ShapeUnion},
            rect::RectShape,
            shape::Shape,
            transf::{IntoTransformed, TransformedShape},
        },
        window::Window,
    },
    math::{
        rect::rect,
        transf::Transform,
        vector::{vec2, IVec2, Vec2},
    },
};
use tracing::info;
use winit::{event::MouseButton, keyboard::KeyCode};

fn main() {
    let width = 220u32;
    let height = 120u32;

    let window = pollster::block_on(Window::init(width, height, 5));

    let mut r = 0.0;

    let shape = RectShape::new(vec2(80, 10)).colored(Color::RED)
        + Circle::new(8).moved(vec2(40.0, 0.0)).colored(Color::GREEN)
        + Circle::new(8).moved(vec2(-40.0, 0.0)).colored(Color::BLUE);

    window.run(|frame| {
        frame.clear(Color::BLACK);

        if frame.is_key_pressed(KeyCode::ArrowLeft) {
            r -= 0.01;
        }

        if frame.is_key_pressed(KeyCode::ArrowRight) {
            r += 0.01;
        }

        frame.draw_shape(shape.stroked(5).rotated(r).moved(vec2(110.0, 60.0)));
    })
}
