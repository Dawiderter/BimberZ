use winit::{keyboard::{PhysicalKey, KeyCode}, event::{Event, MouseButton, WindowEvent, ElementState, DeviceEvent, KeyEvent}};

pub use winit::keyboard::KeyCode as Key;
pub use winit::event::MouseButton as Mouse;

/// Referencja
/// VirtualKeyCode z Winit
/// Łącznie 162 klawisze
/// pub enum VirtualKeyCode {
///     Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, Key0,
///     A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
///     Escape,
///     F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24,
///     Snapshot, Scroll, Pause, Insert, Home, Delete, End, PageDown, PageUp,
///     Left, Up, Right, Down, Back,
///     Return,
///     Space,
///     Compose, Caret,
///     Numlock, Numpad0, Numpad1, Numpad2, Numpad3, Numpad4, Numpad5, Numpad6, Numpad7, Numpad8, Numpad9,
///     NumpadAdd, NumpadDivide, NumpadDecimal, NumpadComma, NumpadEnter, NumpadEquals, NumpadMultiply, NumpadSubtract,
///     AbntC1, AbntC2, Apostrophe, Apps, Asterisk, At, Ax,
///     Backslash, Calculator, Capital, Colon, Comma, Convert, Equals, Grave, Kana, Kanji,
///     LAlt, LBracket, LControl, LShift, LWin,
///     Mail, MediaSelect, MediaStop, Minus, Mute, MyComputer,
///     NavigateForward, NavigateBackward,
///     NextTrack, NoConvert, OEM102, Period, PlayPause, Plus, Power, PrevTrack,
///     RAlt, RBracket, RControl, RShift, RWin,
///     Semicolon, Slash, Sleep, Stop, Sysrq,
///     Tab,
///     Underline, Unlabeled, VolumeDown, VolumeUp, Wake,
///     WebBack, WebFavorites, WebForward, WebHome, WebRefresh, WebSearch, WebStop,
///     Yen, Copy, Paste, Cut,
/// }
///
/// I MouseButton
/// pub enum MouseButton {
///     Left,
///     Right,
///     Middle,
///     Other(u16),
/// }

const NUM_OF_KEYS: usize = 162;
const NUM_OF_MOUSE_BUTTONS: usize = 4;

#[derive(Debug)]
pub struct Input {
    pressed_key: [bool; NUM_OF_KEYS],
    tapped_key: [bool; NUM_OF_KEYS],
    pressed_mouse: [bool; NUM_OF_MOUSE_BUTTONS],
    tapped_mouse: [bool; NUM_OF_MOUSE_BUTTONS],
    mouse_position: (f64,f64),
    mouse_motion: (f64,f64),
}

impl Input {
    pub fn init() -> Self {
        Self {
            pressed_key: [false; NUM_OF_KEYS],
            tapped_key: [false; NUM_OF_KEYS],
            pressed_mouse: [false; NUM_OF_MOUSE_BUTTONS],
            tapped_mouse: [false; NUM_OF_MOUSE_BUTTONS],
            mouse_position: (0.0,0.0),
            mouse_motion: (0.0,0.0),
        }
    }

    pub fn is_key_pressed(&self, code : KeyCode) -> bool{
        self.pressed_key[code as usize]
    }

    pub fn is_mouse_pressed(&self, button: MouseButton) -> bool{
        self.pressed_mouse[Self::button_id(button)]
    }

    pub fn on_key_tap(&self, code : KeyCode) -> bool{
        self.tapped_key[code as usize]
    }

    pub fn on_mouse_tap(&self, button: MouseButton) -> bool{
        self.tapped_mouse[Self::button_id(button)]
    }

    pub fn mouse_position(&self) -> (f64,f64) {
        self.mouse_position
    }

    pub fn mouse_motion(&self) -> (f64,f64) {
        self.mouse_motion
    }

    pub fn clear_tapped(&mut self) {
        self.tapped_key = [false; NUM_OF_KEYS];
        self.tapped_mouse = [false; NUM_OF_MOUSE_BUTTONS];
        self.mouse_motion = (0.0,0.0);
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
            },
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                self.mouse_position = (position.x, position.y);
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        event : KeyEvent { physical_key : PhysicalKey::Code(keycode) , state, ..},
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
            },
            Event::DeviceEvent {
                event:
                    DeviceEvent::MouseMotion { delta },
                ..
            } => {
                self.mouse_motion.0 += delta.0;
                self.mouse_motion.1 += delta.1;
            },
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
