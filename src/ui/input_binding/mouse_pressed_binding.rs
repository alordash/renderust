use minifb::MouseButton;

pub enum MousePressMode {
    Down,
    Up
}

pub struct MousePressedBinding<'a> {
    pub button: MouseButton,
    pub press_mode: MousePressMode,
    pub callback: Box<dyn FnMut(f32, f32) + 'a>,
}

impl<'a> MousePressedBinding<'a> {
    pub fn new<F: 'a + FnMut(f32, f32)>(button: MouseButton, press_mode: MousePressMode, callback: F) -> MousePressedBinding<'a> {
        MousePressedBinding {
            button,
            press_mode,
            callback: Box::new(callback),
        }
    }
}
