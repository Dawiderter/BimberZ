use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

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

    let mut show_window = Arc::new(Mutex::new(false));
    let mut window_name = Arc::new(Mutex::new("Test".to_string()));

    window.run(|input, u, scene, egui_ctx| {
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

        egui::Window::new("Inspector").show(egui_ctx, |ctx| {
            if ctx.button("Box").clicked() {
                scene.shape = sdbox(half_diag)
                    .rounded(0.2)
                    .smooth_union(sdsphere(1.0).translated(vec3(-1.5, 0.0, 0.0)), 0.25)
                    .smooth_union(
                        sdbox(vec3(0.4, 0.4, 0.4))
                            .translated(vec3(0.0, 1.5, 0.0))
                            .rounded(0.2),
                        0.1,
                    )
                    .rotated(rotation)
                    .translated(translation);
                scene.has_changed = true;
            }
            if ctx.button("Sphere").clicked() {
                scene.shape = sdsphere(radius).translated(translation);
                scene.has_changed = true;
            }
        });

        let check_clone = show_window.clone();
        let name_clone = window_name.clone();
        egui_ctx.show_viewport_deferred(
            egui::ViewportId::from_hash_of("Contorl"),
            egui::ViewportBuilder::default()
                .with_title("Control")
                .with_inner_size((200.0, 200.0)),
            move |ctx, _| {
                egui::Window::new("Control").show(ctx, |ui| {
                    let mut check = *check_clone.lock().unwrap();
                    let mut name = name_clone.lock().unwrap().clone();
                    ui.checkbox(&mut check, "Show window?");
                    ui.text_edit_singleline(&mut name);
                    *check_clone.lock().unwrap() = check;
                    *name_clone.lock().unwrap() = name;
                });
            },
        );

        if *show_window.lock().unwrap() {
            egui_ctx.show_viewport_deferred(
                egui::ViewportId::from_hash_of("Tertiary"),
                egui::ViewportBuilder::default()
                    .with_title(window_name.lock().unwrap().clone())
                    .with_inner_size((200.0, 200.0)),
                move |ctx, _| {
                    egui::Window::new("Oh it worked").show(ctx, |ui| {
                        ui.label("Yay");
                    });
                },
            );
        }

        let now = start.elapsed().as_secs_f32();

        u[rotation] = Quat::from_euler(glam::EulerRot::XYZ, now / 3.0, now / 4.0, now / 2.0);
    })
}
