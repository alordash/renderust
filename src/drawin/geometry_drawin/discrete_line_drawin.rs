use crate::{
    discretization::geometry_discretization::discrete_line_x_axis_calculator::DiscreteLineXAxisCalculator,
    drawin::{color::Color, drawable::Drawable},
    geometry::{
        primitives::discrete_line::DiscreteLine, primitives::discrete_point::DiscretePoint,
    },
};

impl Drawable for DiscreteLine {
    fn draw(&self, canvas: &mut crate::drawin::draw_buffer::DrawBuffer, color: &Color) {
        let line_iterator = self.into_iter();
        for point in line_iterator {
            canvas[point] = *color;
        }
    }
}
