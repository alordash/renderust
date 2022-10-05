use super::{color::Color, draw_buffer::DrawBuffer};

pub trait Drawable {
    fn draw(&self, canvas: &mut DrawBuffer, color: Option<&Color>);
    fn fill(&self, canvas: &mut DrawBuffer, color: Option<&Color>) {}
}
