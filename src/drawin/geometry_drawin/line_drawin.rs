use crate::{
    discretization::geometry_discretization::line_x_axis_calculator::LineXAxisCalculator,
    drawin::{color::Color, drawable::Drawable},
    geometry::{
        primitives::line::Line, primitives::point::Point,
    },
};

impl Drawable for Line {
    fn draw(&self, canvas: &mut crate::drawin::draw_buffer::DrawBuffer, color: &Color) {
        let line_iterator = self.into_iter();
        for point in line_iterator {
            canvas[point] = *color;
        }
    }
}
