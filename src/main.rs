use bimberz::drawing::{backend::window::Window, framebuffer::FrameBuffer, color::Color};


fn main() {
    let window = pollster::block_on(Window::init(100,100,10));

    let mut i = 0;

    window.run(|frame| {
        frame.clear(Color::BLACK);

        let mut x = 0;
        let mut y = 0;
        let mut dir_x = 1;
        let mut dir_y = 0;
        let mut cycles: u32 = 0;

        for j in 0..=i {
            let color = if (i - j) % 512 >= 256 {
                255 - (i - j) % 256
            } else {
                (i - j) % 256
            } as u8;

            frame.put_pixel(x, y, Color::new(255 - color, color, 255, 255));

            if y == (0 + cycles * 2) && dir_y == -1 {
                dir_x = 1;
                dir_y = 0;
            } else if x == (frame.width() - 1 - cycles * 2) && dir_x == 1 {
                dir_x = 0;
                dir_y = 1;
            } else if y == (frame.height() - 1 - cycles * 2) && dir_y == 1 {
                dir_x = -1;
                dir_y = 0;
            } else if x == (0 + cycles * 2) && dir_x == -1 {
                dir_x = 0;
                dir_y = -1;
                cycles += 1;
            }

            x = (x as isize + dir_x) as u32;
            y = (y as isize + dir_y) as u32;
        }

        i += 1;

        if cycles * 4 >= frame.width() - 1 || cycles * 4 >= frame.height() - 1 {
            i = 0;
        }
    })
}
