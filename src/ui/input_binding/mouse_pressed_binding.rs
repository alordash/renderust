use minifb::MouseButton;

pub struct MousePressedBinding<'a> {
    pub button: MouseButton,
    pub callback: Box<dyn FnMut() + 'a>,
}

impl<'a> MousePressedBinding<'a> {
    pub fn new<F: 'a + FnMut()>(button: MouseButton, callback: F) -> MousePressedBinding<'a> {
        MousePressedBinding {
            button,
            callback: Box::new(callback),
        }
    }
}
