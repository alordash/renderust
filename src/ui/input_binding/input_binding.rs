use minifb::Window;

use super::{
    keyboard_binding::{KeyBindingKind, KeyboardBinding},
    mouse_position_binding::MousePositionBinding,
    mouse_pressed_binding::{MousePressMode, MousePressedBinding},
    mouse_scroll_binding::MouseScrollBinding,
};

pub enum InputBinding<'a> {
    MousePressed(MousePressedBinding<'a>),
    MousePosition(MousePositionBinding<'a>),
    MouseScroll(MouseScrollBinding<'a>),
    Keyboard(KeyboardBinding<'a>),
}

impl<'a> InputBinding<'a> {
    pub fn handle_inputs(window: &Window, input_bindings: &mut [InputBinding]) {
        for input_binding in input_bindings.iter_mut() {
            match input_binding {
                InputBinding::MousePressed(mouse_pressed) => {
                    let should_be_down = match mouse_pressed.press_mode {
                        MousePressMode::Down => true,
                        MousePressMode::Up => false,
                    };
                    if window.get_mouse_down(mouse_pressed.button) == should_be_down {
                        let (x, y) = window.get_mouse_pos(minifb::MouseMode::Pass).unwrap();
                        (mouse_pressed.callback)(x, y);
                    }
                }
                InputBinding::MousePosition(mouse_position) => {
                    if let Some((x, y)) = window.get_mouse_pos(mouse_position.mode) {
                        (mouse_position.callback)(x, y);
                    }
                }
                InputBinding::MouseScroll(mouse_scroll) => {
                    if let Some((x, y)) = window.get_scroll_wheel() {
                        (mouse_scroll.callback)(x, y);
                    }
                }
                InputBinding::Keyboard(keyboard) => match keyboard.kind {
                    KeyBindingKind::KeyDown => {
                        if window.is_key_down(keyboard.key) {
                            (keyboard.callback)()
                        }
                    }
                    KeyBindingKind::KeyPressed(repeat) => {
                        if window.is_key_pressed(keyboard.key, repeat) {
                            (keyboard.callback)()
                        }
                    }
                    KeyBindingKind::KeyReleased => {
                        if window.is_key_released(keyboard.key) {
                            (keyboard.callback)()
                        }
                    }
                },
            }
        }
    }
}
