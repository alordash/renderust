use minifb::{Key, KeyRepeat, Window};

pub enum InputBindingKind {
    KeyDown,
    KeyPressed(KeyRepeat),
    KeyReleased,
}

pub struct InputBinding<'a> {
    pub key: Key,
    pub kind: InputBindingKind,
    pub callback: Box<dyn FnMut() + 'a>,
}

impl<'a> InputBinding<'a> {
    pub fn new<F: 'a + FnMut()>(key: Key, kind: InputBindingKind, callback: F) -> InputBinding<'a> {
        InputBinding {
            key,
            kind,
            callback: Box::new(callback),
        }
    }

    pub fn handle_inputs(window: &Window, input_bindings: &mut [InputBinding]) {
        for input_binding in input_bindings.iter_mut() {
            match input_binding.kind {
                InputBindingKind::KeyDown => {
                    if window.is_key_down(input_binding.key) {
                        (input_binding.callback)()
                    }
                }
                InputBindingKind::KeyPressed(repeat) => {
                    if window.is_key_pressed(input_binding.key, repeat) {
                        (input_binding.callback)()
                    }
                }
                InputBindingKind::KeyReleased => {
                    if window.is_key_released(input_binding.key) {
                        (input_binding.callback)()
                    }
                }
            }
        }
    }
}
