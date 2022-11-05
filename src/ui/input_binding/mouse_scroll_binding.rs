pub struct MouseScrollBinding<'a> {
    pub callback: Box<dyn FnMut(f32, f32) + 'a>,
}

impl<'a> MouseScrollBinding<'a> {
    pub fn new<F: 'a + FnMut(f32, f32)>(callback: F) -> MouseScrollBinding<'a> {
        MouseScrollBinding {
            callback: Box::new(callback),
        }
    }
}
