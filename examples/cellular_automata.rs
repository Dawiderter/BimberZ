use std::f32::consts::PI;

use bimberz::{
    drawing::{
        color::Color,
        renderer::{
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

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    Sand,
    Air,
    Wall,
}

fn get(board: &[State], width: isize, height: isize, x: isize, y: isize) -> State {
    if 0 <= x && x < width && 0 <= y && y < height {
        board[(y * width + x) as usize]
    } else {
        State::Wall
    }
}

fn sim(board: &[State], target: &mut [State], width: isize, height: isize) {
    for y in 0..height {
        for x in 0..width {
            let middle = board[(y * width + x) as usize];
            let down = get(board, width, height, x, y + 1);

            if matches!((middle, down), (State::Sand, State::Air)) {
                target[((y + 1) * width + x) as usize] = State::Sand;
                continue;
            }

            let down_right = get(board, width, height, x + 1, y + 1);

            if matches!(
                (middle, down, down_right),
                (State::Sand, State::Sand, State::Air)
            ) {
                target[((y + 1) * width + x + 1) as usize] = State::Sand;
                continue;
            }

            let down_left = get(board, width, height, x - 1, y + 1);

            if matches!(
                (middle, down, down_left),
                (State::Sand, State::Sand, State::Air)
            ) {
                target[((y + 1) * width + x - 1) as usize] = State::Sand;
                continue;
            }

            if middle == State::Sand {
                target[(y * width + x) as usize] = State::Sand;
                continue;
            }
        }
    }
}

fn main() {
    let width = 100;
    let height = 100;

    let window = pollster::block_on(Window::init(width as u32, height as u32, 5));

    let mut board = vec![State::Air; width * height];
    let mut swap_board = vec![State::Air; width * height];
    let step = 10;
    let mut i = 0;

    window.run(|frame| {
        let (mouse_x, mouse_y) = frame.mouse_position();
        let mouse_x = mouse_x as isize / 5;
        let mouse_y = mouse_y as isize / 5;

        if frame.is_mouse_pressed(MouseButton::Left)
            && 0 <= mouse_x
            && mouse_x < width as isize
            && 0 <= mouse_y
            && mouse_y < height as isize
        {
            board[((mouse_y * width as isize) + mouse_x) as usize] = State::Sand;
        }

        if i >= step {
            i -= step;
            swap_board.fill(State::Air);
            sim(&board, &mut swap_board, width as isize, height as isize);
            std::mem::swap(&mut board, &mut swap_board);
        }

        frame.clear(Color::BLACK);

        for y in 0..height {
            for x in 0..width {
                if board[y * width + x] == State::Sand {
                    frame.put_pixel(x as u32, y as u32, Color::WHITE);
                }
            }
        }

        i += 1;
    })
}
