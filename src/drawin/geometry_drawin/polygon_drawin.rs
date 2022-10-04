use std::ops::Range;

use crate::{
    discretization::geometry_discretization::line_x_axis_calculator::LineXAxisCalculator,
    drawin::drawable::Drawable,
    geometry::primitives::{line::Line, polygon::Polygon},
};

struct PolygonFillingRange<'a> {
    range: Range<isize>,
    line_calculators: Vec<&'a LineXAxisCalculator>,
}

impl<'a> Iterator for PolygonFillingRange<'a> {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        self.range.next()
    }
}

impl<const N: usize> Drawable for Polygon<N> {
    default fn draw(
        &self,
        canvas: &mut crate::drawin::draw_buffer::DrawBuffer,
        color: &crate::drawin::color::Color,
    ) {
        self.get_perimeter_lines()
            .into_iter()
            .for_each(|l| l.draw(canvas, color));
    }

    default fn fill(
        &self,
        canvas: &mut crate::drawin::draw_buffer::DrawBuffer,
        color: &crate::drawin::color::Color,
    ) {
        let mut lines = self.get_perimeter_lines();
        lines.iter_mut().for_each(Line::order_by_x);

        let line_calculators: Vec<LineXAxisCalculator> =
            lines.into_iter().map(LineXAxisCalculator::from).collect();

        let mut x_sorted_points = self.points.clone();
        x_sorted_points.sort_unstable_by(|a, b| a.x.cmp(&b.x));
        let polygon_filling_ranges = x_sorted_points.windows(2).map(|two_points| unsafe {
            let range = two_points.get_unchecked(0).x..two_points.get_unchecked(1).x;
            let suitable_line_calculators: Vec<&LineXAxisCalculator> = line_calculators
                .iter()
                .filter(|lc| {
                    let lc_range = lc.get_x_calculation_range();
                    lc_range.start <= range.start && range.end <= lc_range.end
                })
                .collect();
            PolygonFillingRange {
                line_calculators: suitable_line_calculators,
                range,
            }
        });

        for polygon_filling_range in polygon_filling_ranges.into_iter() {
            for x in polygon_filling_range.range {
                let mut y_and_colors: Vec<_> = polygon_filling_range
                    .line_calculators
                    .iter()
                    .map(|v| v.calculate_y_and_color_value(x))
                    .collect();
                y_and_colors.sort_unstable_by(|a, b| a.0.cmp(&b.0));

                for two_ys in y_and_colors.chunks_exact(2) {
                    let mut y_and_color1 = *unsafe { two_ys.get_unchecked(0) };
                    let mut y_and_color2 = *unsafe { two_ys.get_unchecked(1) };

                    if y_and_color1.0 > y_and_color2.0 {
                        (y_and_color1, y_and_color2) = (y_and_color2, y_and_color1);
                    }
                    let (y1, y2) = (y_and_color1.0, y_and_color2.0);
                    let (color1, color2) = (y_and_color1.1, y_and_color2.1);
                    let d = y2 - y1;

                    for y in y1..y2 {
                        let new_color = color1.interpolate(color2, (y - y1) as i32, d as i32);
                        canvas[(x as usize, y as usize)] = new_color;
                    }
                }
            }
        }
    }
}
