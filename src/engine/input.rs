use winit::{
    event::{DeviceEvent, ElementState, Event, KeyEvent, MouseButton, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

pub use winit::event::MouseButton as Mouse;
pub use winit::keyboard::KeyCode as Key;

/// Referencja
/// KeyCode z Winit
/// Łącznie 194 klawisze
///
/// I MouseButton
/// pub enum MouseButton {
///     Left,
///     Right,
///     Middle,
///     Other(u16),
/// }

const NUM_OF_KEYS: usize = 194;
const NUM_OF_MOUSE_BUTTONS: usize = 4;

#[derive(Debug)]
pub struct Input {
    pressed_key: [bool; NUM_OF_KEYS],
    tapped_key: [bool; NUM_OF_KEYS],
    pressed_mouse: [bool; NUM_OF_MOUSE_BUTTONS],
    tapped_mouse: [bool; NUM_OF_MOUSE_BUTTONS],
    mouse_position: (f64, f64),
    mouse_motion: (f64, f64),
}

impl Input {
    pub fn new() -> Self {
        Self {
            pressed_key: [false; NUM_OF_KEYS],
            tapped_key: [false; NUM_OF_KEYS],
            pressed_mouse: [false; NUM_OF_MOUSE_BUTTONS],
            tapped_mouse: [false; NUM_OF_MOUSE_BUTTONS],
            mouse_position: (0.0, 0.0),
            mouse_motion: (0.0, 0.0),
        }
    }

    pub fn is_key_pressed(&self, code: KeyCode) -> bool {
        self.pressed_key[code as usize]
    }

    pub fn is_mouse_pressed(&self, button: MouseButton) -> bool {
        self.pressed_mouse[Self::button_id(button)]
    }

    pub fn on_key_tap(&self, code: KeyCode) -> bool {
        self.tapped_key[code as usize]
    }

    pub fn on_mouse_tap(&self, button: MouseButton) -> bool {
        self.tapped_mouse[Self::button_id(button)]
    }

    pub fn mouse_position(&self) -> (f64, f64) {
        self.mouse_position
    }

    pub fn mouse_motion(&self) -> (f64, f64) {
        self.mouse_motion
    }

    pub fn clear_tapped(&mut self) {
        self.tapped_key = [false; NUM_OF_KEYS];
        self.tapped_mouse = [false; NUM_OF_MOUSE_BUTTONS];
        self.mouse_motion = (0.0, 0.0);
    }

    pub fn register_event(&mut self, event: &Event<()>) {
        match event {
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                let button_id = Self::button_id(*button);
                match state {
                    ElementState::Pressed => {
                        if !self.pressed_mouse[button_id] {
                            self.tapped_mouse[button_id] = true;
                        }
                        self.pressed_mouse[button_id] = true;
                    }
                    ElementState::Released => {
                        self.pressed_mouse[button_id] = false;
                    }
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                self.mouse_position = (position.x, position.y);
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                physical_key: PhysicalKey::Code(keycode),
                                state,
                                ..
                            },
                        ..
                    },
                ..
            } => {
                let key_id = *keycode as usize;
                match state {
                    ElementState::Pressed => {
                        if !self.pressed_key[key_id] {
                            self.tapped_key[key_id] = true;
                        }
                        self.pressed_key[key_id] = true;
                    }
                    ElementState::Released => {
                        self.pressed_key[key_id] = false;
                    }
                }
            }
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                self.mouse_motion.0 += delta.0;
                self.mouse_motion.1 += delta.1;
            }
            _ => {}
        }
    }

    fn button_id(button: MouseButton) -> usize {
        match button {
            MouseButton::Left => 0,
            MouseButton::Right => 1,
            MouseButton::Middle => 2,
            _ => 3,
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}
