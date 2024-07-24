use bimberz::engine::{
    renderer::scene::{sdbox, sphere},
    window::Window,
};
use glam::vec3;
use winit::keyboard::KeyCode;

fn main() {
    let width = 600u32;
    let height = 600u32;

    let mut window = pollster::block_on(Window::new(width, height, 1));

    let radius = window.uniforms().bind(1.0);
    let half_diag = window.uniforms().bind(vec3(1.0, 1.0, 1.0));

    window.scene().shape = sphere(radius);

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

        if input.on_key_tap(KeyCode::KeyB) {
            scene.shape = sdbox(half_diag);
            scene.has_changed = true;
        }
        if input.on_key_tap(KeyCode::KeyC) {
            scene.shape = sphere(radius);
            scene.has_changed = true;
        }
    })
}
