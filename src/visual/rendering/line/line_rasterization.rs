use crate::{
    math::geometry::primitives::line::Line,
    visual::{color::color::Color, drawing_buffer::DrawingBuffer},
};

pub fn draw_line(line: &Line, canvas: &mut DrawingBuffer, color: Option<&Color>) {
    let line_iterator = line.into_iter();
    for point in line_iterator {
        canvas[(point.x as usize, point.y as usize)] = unsafe { *color.unwrap_unchecked() };
    }
}
