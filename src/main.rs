use std::time::Instant;

use bimberz::engine::{
    renderer::scene::{sdbox, sdsphere},
    window::Window,
};
use glam::{vec3, Quat};
use winit::keyboard::KeyCode;

fn main() {
    let width = 600u32;
    let height = 600u32;

    let mut window = pollster::block_on(Window::new(width, height, 1));

    let radius = window.uniforms().bind(1.0);
    let half_diag = window.uniforms().bind(vec3(1.0, 1.0, 1.0));
    let translation = window.uniforms().bind(vec3(0.0, 0.0, 0.0));
    let rotation = window.uniforms().bind(Quat::IDENTITY);

    window.scene().shape = sdsphere(radius).translated(translation);

    let start = Instant::now();

    window.run(|input, u, scene| {
        if input.is_key_pressed(KeyCode::KeyD) {
            u[radius] += 0.01;
            u[half_diag].x += 0.01;
        }
        if input.is_key_pressed(KeyCode::KeyA) {
            u[radius] -= 0.01;
            u[half_diag].x -= 0.01;
        }
        if input.is_key_pressed(KeyCode::KeyW) {
            u[radius] += 0.01;
            u[half_diag].y += 0.01;
        }
        if input.is_key_pressed(KeyCode::KeyS) {
            u[radius] -= 0.01;
            u[half_diag].y -= 0.01;
        }

        if input.is_key_pressed(KeyCode::ArrowRight) {
            u[translation].x += 0.01;
        }
        if input.is_key_pressed(KeyCode::ArrowLeft) {
            u[translation].x -= 0.01;
        }
        if input.is_key_pressed(KeyCode::ArrowUp) {
            u[translation].y += 0.01;
        }
        if input.is_key_pressed(KeyCode::ArrowDown) {
            u[translation].y -= 0.01;
        }
        if input.is_key_pressed(KeyCode::KeyI) {
            u[translation].z += 0.01;
        }
        if input.is_key_pressed(KeyCode::KeyK) {
            u[translation].z -= 0.01;
        }

        if input.on_key_tap(KeyCode::KeyB) {
            scene.shape = sdbox(half_diag)
                .rounded(0.5)
                .smooth_union(sdsphere(1.0).translated(vec3(-1.5, 0.0, 0.0)), 0.25)
                .smooth_union(
                    sdbox(vec3(0.8, 0.8, 0.8))
                        .translated(vec3(0.0, 1.5, 0.0))
                        .rounded(0.2),
                    0.1,
                )
                .rotated(rotation)
                .translated(translation);
            scene.has_changed = true;
        }
        if input.on_key_tap(KeyCode::KeyC) {
            scene.shape = sdsphere(radius).translated(translation);
            scene.has_changed = true;
        }

        let now = start.elapsed().as_secs_f32();

        u[rotation] = Quat::from_euler(glam::EulerRot::XYZ, now / 3.0, now / 4.0, now / 2.0);
    })
}
