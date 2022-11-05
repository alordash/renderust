use minifb::MouseMode;

pub struct MousePositionBinding<'a> {
    pub mode: MouseMode,
    pub callback: Box<dyn FnMut(f32, f32) + 'a>,
}

impl<'a> MousePositionBinding<'a> {
    pub fn new<F: 'a + FnMut(f32, f32)>(mode: MouseMode, callback: F) -> MousePositionBinding<'a> {
        MousePositionBinding {
            mode,
            callback: Box::new(callback),
        }
    }
}
