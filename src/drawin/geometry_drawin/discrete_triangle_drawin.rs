use crate::{
    discretization::geometry_discretization::discrete_line_x_axis_calculator::DiscreteLineXAxisCalculator,
    drawin::{color::Color, drawable::Drawable},
    geometry::primitives::{
        discrete_line::DiscreteLine, discrete_point::DiscretePoint,
        polygons::discrete_triangle::DiscreteTriangle,
    },
};

impl Drawable for DiscreteTriangle {
    fn draw(
        &self,
        canvas: &mut crate::drawin::draw_buffer::DrawBuffer,
        color: &crate::drawin::color::Color,
    ) {
        let (left_point_idx, left_point) = self
            .points
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.x.cmp(&b.1.x))
            .unwrap();
        let (right_point_idx, right_point) = self
            .points
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.x.cmp(&b.1.x))
            .unwrap();

        let (_, middle_point) = unsafe {
            *self
                .points
                .iter()
                .enumerate()
                .filter(|(idx, _)| *idx != left_point_idx && *idx != right_point_idx)
                .collect::<Vec<_>>()
                .get_unchecked(0)
        };

        let long_line = DiscreteLine {
            begin: *left_point,
            end: *right_point,
        };
        let left_line = DiscreteLine {
            begin: *left_point,
            end: *middle_point,
        };
        let right_line = DiscreteLine {
            begin: *middle_point,
            end: *right_point,
        };

        let long_line_calculator = DiscreteLineXAxisCalculator::from(long_line);
        let left_line_calculator = DiscreteLineXAxisCalculator::from(left_line);
        let right_line_calculator = DiscreteLineXAxisCalculator::from(right_line);

        let mut short_line_calculator = &left_line_calculator;

        let mut short_line_left_iters = short_line_calculator.get_x_calculation_range().len();

        if short_line_left_iters == 0 {
            short_line_calculator = &right_line_calculator;
        }

        for x in left_point.x..right_point.x {
            let p1 = DiscretePoint {
                x,
                y: long_line_calculator.calculate_y_value(x),
            };
            let p2 = DiscretePoint {
                x,
                y: short_line_calculator.calculate_y_value(x),
            };

            let y_min = p1.y.min(p2.y);
            let y_max = p1.y.max(p2.y);

            for y in y_min..y_max {
                canvas[(x as usize, y as usize)] = *color;
            }

            if short_line_left_iters > 0 {
                short_line_left_iters -= 1;
                if short_line_left_iters == 0 {
                    short_line_calculator = &right_line_calculator;
                }
            }
        }
    }
}
