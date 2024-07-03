use bimberz::{
    drawing::{
        color::Color,
        renderer::{
            circle::Circle,
            rect::RectShape, transf::IntoTransformed, coloring::IntoColored,
        },
        window::Window,
    },
    math::{
        vector::vec2,
    },
};

use winit::keyboard::KeyCode;

fn main() {
    let width = 1000u32;
    let height = 1000u32;
    let scale = 1;

    let window = pollster::block_on(Window::init(width, height, scale));

    let circle = Circle::new(width as i32/3).colored(Color::RED).moved(vec2(width/2, height/2).to_float());

    let mut m_rect = RectShape::new(vec2(width/6, width/12)).colored(Color::BLUE).moved(vec2(0.0, 0.0));

    let mut mode = 0;
    let mut r = 0.0;

    window.run(|frame| {
        frame.clear(Color::BLACK);

        let (mouse_x, mouse_y) = frame.mouse_position();
        let mouse_pos = vec2((mouse_x) as i32, (mouse_y) as i32) / scale as i32;

        m_rect.transform.translation = mouse_pos.to_float();

        if frame.on_key_tap(KeyCode::Space) {
            mode += 1;
            mode %= 3;
        }

        if frame.is_key_pressed(KeyCode::ArrowLeft) {
            r -= 0.01;
        }

        if frame.is_key_pressed(KeyCode::ArrowRight) {
            r += 0.01;
        }

        m_rect.transform.rotation = r;

        match mode {
            0 => frame.draw_shape((circle - m_rect)),
            1 => frame.draw_shape((circle * m_rect)),
            2 => frame.draw_shape((circle + m_rect)),
            _ => unreachable!()
        }
    })
}
