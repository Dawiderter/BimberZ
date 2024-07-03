use std::f32::consts::PI;

use bimberz::{
    drawing::{
        color::Color,
        renderer::{
            circle::Circle,
            rect::RectShape,
            shape::Shape,
            transf::TransformedShape,
        },
        window::Window,
    },
    math::{
        rect::rect,
        transf::Transform,
        vector::vec2,
    },
};
use winit::keyboard::KeyCode;

fn main() {
    let width = 220u32;
    let height = 120u32;

    let window = pollster::block_on(Window::init(width, height, 5));

    let margin = 10u32;

    let rectsh = RectShape {
        rect: rect(vec2(-1, -7), vec2(1, 7)),
    };
    let pad1 = RectShape {
        rect: rect(vec2(3, -11), vec2(4, 11)),
    };
    let pad2 = RectShape {
        rect: rect(vec2(-4, -11), vec2(-3, 11)),
    };
    let mut p1 = TransformedShape {
        transform: Transform::mov(vec2(margin, height / 2).to_float()),
        a: (rectsh + 2) | pad1,
    };
    let mut p2 = TransformedShape {
        transform: Transform::mov(vec2(width - 1 - margin, height / 2).to_float()),
        a: (rectsh + 2) | pad2,
    };

    let circle = Circle {
        center: vec2(0, 0),
        radius: 2,
    };
    let mut ball = TransformedShape {
        transform: Transform::mov(vec2(width / 2, height / 2).to_float()),
        a: circle,
    };
    let mut ball_dir = vec2(1.0, 1.0).norm();
    let ball_speed = 0.8;

    window.run(|frame| {
        frame.clear(Color::BLACK);

        if frame.is_key_pressed(KeyCode::KeyW) {
            p1.transform.translation.y -= 1.0;
        }
        if frame.is_key_pressed(KeyCode::KeyS) {
            p1.transform.translation.y += 1.0;
        }
        if frame.is_key_pressed(KeyCode::KeyA) {
            p1.transform.rotation += 0.03;
        }
        if frame.is_key_pressed(KeyCode::KeyD) {
            p1.transform.rotation -= 0.03;
        }
        p1.transform.translation.y = p1.transform.translation.y.clamp(0.0, frame.height() as f32 - 1.0);

        if frame.is_key_pressed(KeyCode::KeyU) {
            p2.transform.translation.y -= 1.0;
        }
        if frame.is_key_pressed(KeyCode::KeyJ) {
            p2.transform.translation.y += 1.0;
        }
        if frame.is_key_pressed(KeyCode::KeyH) {
            p2.transform.rotation += 0.03;
        }
        if frame.is_key_pressed(KeyCode::KeyK) {
            p2.transform.rotation -= 0.03;
        }
        p2.transform.translation.y = p2.transform.translation.y.clamp(0.0, frame.height() as f32 - 1.0);

        let mut ball_pos = ball.transform.translation;
        let ball_pos_int = ball_pos.to_int();

        if ball_pos_int.y <= 0 || ball_pos_int.y >= frame.height() as i32 - 1 {
            ball_dir.y = -ball_dir.y;
        }

        if (p1 + 2).contains(ball_pos_int) {
            let (s,c) = p1.transform.rotation.sin_cos();
            let n = vec2(c,s);
            ball_dir = ball_dir - n * (ball_dir.dot(n)) * 2.0;

            while (p1 + 2).contains(ball_pos.to_int()) {
                ball_pos = ball_pos + ball_dir * ball_speed;
            }
        }

        if (p2 + 2).contains(ball_pos_int) {
            let (s,c) = (p2.transform.rotation + PI).sin_cos();
            let n = vec2(c,s);
            ball_dir = ball_dir - n * (ball_dir.dot(n)) * 2.0;

            while (p2 + 2).contains(ball_pos.to_int()) {
                ball_pos = ball_pos + ball_dir * ball_speed;
            }
        }

        if ball_pos_int.x <= 0 || ball_pos_int.x >= frame.width() as i32 - 1 {
            ball_dir = ball_dir * -1.0;
            ball_pos = vec2(frame.width() / 2, frame.height() / 2).to_float();
        }

        ball_pos = ball_pos + ball_dir * ball_speed;

        ball.transform.translation = ball_pos;

        frame.draw_shape_stroke(p1, Color::WHITE);
        frame.draw_shape_stroke(p2, Color::WHITE);
        frame.draw_shape_filled(ball, Color::WHITE);
    })
}
