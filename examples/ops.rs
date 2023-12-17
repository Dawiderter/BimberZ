use bimberz::{
    drawing::{
        color::Color,
        shapes::{
            circle::Circle,
            rect::RectShape,
        },
        window::Window,
    },
    math::{
        rect::rect,
        vector::vec2,
    },
};

use winit::keyboard::KeyCode;

fn main() {
    let width = 1000u32;
    let height = 1000u32;

    let window = pollster::block_on(Window::init(width, height, 1));

    let circle = Circle {
        center: vec2(500, 500),
        radius: 300,
    };

    let mut m_rect = RectShape {
        rect: rect(vec2(0, 0), vec2(300, 150))
    };

    let mut mode = 0;

    window.run(|frame| {
        frame.clear(Color::BLACK);

        let (mouse_x, mouse_y) = frame.mouse_position();
        let mouse_pos = vec2((mouse_x) as i32, (mouse_y) as i32);

        m_rect.rect.top_left = mouse_pos - vec2(150, 75);
        m_rect.rect.bot_right = mouse_pos + vec2(150, 75);

        if frame.on_key_tap(KeyCode::Space) {
            mode += 1;
            mode %= 3;
        }

        match mode {
            0 => frame.draw_shape_stroke(circle - m_rect, Color::WHITE),
            1 => frame.draw_shape_stroke(circle & m_rect, Color::WHITE),
            2 => frame.draw_shape_stroke(circle | m_rect, Color::WHITE),
            _ => unreachable!()
        }
    })
}
