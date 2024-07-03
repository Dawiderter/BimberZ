use std::f32::consts::PI;

use bimberz::{
    drawing::{
        color::Color, renderer::{scene::Scene, shape::Shape, sphere, torus}, window::Window
    },
    math::{
        rect::rect,
        transf::Transform,
    },
};
use glam::{vec2, Vec2};
use tracing::info;
use winit::{event::MouseButton, keyboard::KeyCode};

fn main() {
    let width = 600u32;
    let height = 600u32;

    let mut window = pollster::block_on(Window::init(width, height, 1));

    let camera_pos = window.frame.bind::<glam::Vec3>(1);
    *camera_pos.val() = glam::Vec3::new(0.0, 0.0, -5.0);

    let mut camera_angle : f32 = 0.0;

    let radius = window.frame.bind::<f32>(0);
    *radius.val() = 1.0;

    let tor = window.frame.bind::<Vec2>(2);
    *tor.val() = vec2(2.0, 0.5);

    window.render_state.update_bindings(&mut window.frame.bindings);

    let scene = Scene { shape: torus(tor.clone()) };

    window.render_state.set_shader(&scene.to_wgsl());

    window.run(|frame| {
        let mut pos = camera_pos.val();
        let mut radius = radius.val();
        let mut tor = tor.val();

        if frame.is_key_pressed(KeyCode::ArrowLeft) {
            tor.x += 0.01;
        }  
        if frame.is_key_pressed(KeyCode::ArrowRight) {
            tor.x -= 0.01;
        } 

        if frame.is_key_pressed(KeyCode::ArrowUp) {
            tor.y -= 0.01;
        }  
        if frame.is_key_pressed(KeyCode::ArrowDown) {
            tor.y += 0.01;
        }

        
        if frame.is_key_pressed(KeyCode::KeyD) {
            camera_angle += 0.01;
        }  
        if frame.is_key_pressed(KeyCode::KeyA) {
            camera_angle -= 0.01;
        }

        pos.x = -5.0 * camera_angle.sin();
        pos.z = -5.0 * camera_angle.cos();
    })
}
