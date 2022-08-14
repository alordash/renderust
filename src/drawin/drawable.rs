use super::{draw_buffer::DrawBuffer, color::Color};

pub trait Drawable {
    fn draw(&self, canvas: &mut DrawBuffer, color: &Color);
}
