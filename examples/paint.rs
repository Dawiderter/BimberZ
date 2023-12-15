use bimberz::drawing::{window::Window, color::Color};
use winit::{keyboard::KeyCode, event::MouseButton};

fn main() {
    let window = pollster::block_on(Window::init(100, 100, 5));

    let mut x = 0;
    let mut y = 0;

    window.run(|frame| {

        let mut dir_x = 0;
        let mut dir_y = 0;
        
        if frame.on_key_tap(KeyCode::Space) {
            frame.clear(Color::BLACK);
        }

        if frame.is_key_pressed(KeyCode::KeyD) {
            dir_x = 1;
        }

        if frame.is_key_pressed(KeyCode::KeyA) {
            dir_x = -1;
        }

        if frame.is_key_pressed(KeyCode::KeyW) {
            dir_y = -1;
        }

        if frame.is_key_pressed(KeyCode::KeyS) {
            dir_y = 1;
        }

        x += dir_x;
        y += dir_y;

        if frame.is_mouse_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = frame.mouse_position();
            x = mouse_x as i32 / 5;
            y = mouse_y as i32 / 5;
        }


        x = x.clamp(0, frame.width() as i32 - 1);
        y = y.clamp(0, frame.height() as i32 - 1);

        frame.put_pixel(x as u32, y as u32, Color::WHITE)
    })
}
