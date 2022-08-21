use crate::{
    discretization::geometry_discretization::discrete_line_x_axis_calculator::DiscreteLineXAxisCalculator,
    drawin::{color::Color, drawable::Drawable},
    geometry::{
        primitives::discrete_line::DiscreteLine, primitives::discrete_point::DiscretePoint,
    },
};

impl Drawable for DiscreteLine {
    fn draw(&self, canvas: &mut crate::drawin::draw_buffer::DrawBuffer, color: &Color) {
        let line_calculator: DiscreteLineXAxisCalculator = (*self).into();
        for x in line_calculator.get_x_calculation_range() {
            let y = line_calculator.calculate_y_value(x);
            canvas[(x as usize, y as usize)] = *color;
        }

        // let line_iterator = self.into_iter();
        // for point in line_iterator {
        //     canvas[point] = *color;
        // }
    }
}
