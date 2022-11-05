use minifb::{Key, KeyRepeat};

pub enum KeyBindingKind {
    KeyDown,
    KeyPressed(KeyRepeat),
    KeyReleased,
}

pub struct KeyboardBinding<'a> {
    pub key: Key,
    pub kind: KeyBindingKind,
    pub callback: Box<dyn FnMut() + 'a>,
}

impl<'a> KeyboardBinding<'a> {
    pub fn new<F: 'a + FnMut()>(key: Key, kind: KeyBindingKind, callback: F) -> KeyboardBinding<'a> {
        KeyboardBinding {
            key,
            kind,
            callback: Box::new(callback),
        }
    }
}
