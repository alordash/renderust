use std::ops::Range;

use crate::{
    discretization::geometry_discretization::discrete_line_x_axis_calculator::DiscreteLineXAxisCalculator,
    drawin::drawable::Drawable,
    geometry::primitives::{discrete_line::DiscreteLine, discrete_polygon::DiscretePolygon},
};

struct PolygonFillingRange<'a> {
    range: Range<isize>,
    line_calculators: Vec<&'a DiscreteLineXAxisCalculator>,
}

impl<'a> Iterator for PolygonFillingRange<'a> {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        self.range.next()
    }
}

impl<const N: usize> Drawable for DiscretePolygon<N> {
    default fn draw(
        &self,
        canvas: &mut crate::drawin::draw_buffer::DrawBuffer,
        color: &crate::drawin::color::Color,
    ) {
        let mut lines = self.get_perimeter_lines();
        lines.iter_mut().for_each(DiscreteLine::order_by_x);

        let line_calculators: Vec<DiscreteLineXAxisCalculator> = lines
            .into_iter()
            .map(DiscreteLineXAxisCalculator::from)
            .collect();

        let mut x_sorted_points = self.points.clone();
        x_sorted_points.sort_unstable_by(|a, b| a.x.cmp(&b.x));
        let polygon_filling_ranges = x_sorted_points.windows(2).map(|two_points| unsafe {
            let range = two_points.get_unchecked(0).x..two_points.get_unchecked(1).x;
            let suitable_line_calculators: Vec<&DiscreteLineXAxisCalculator> = line_calculators
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
                let mut ys: Vec<isize> = polygon_filling_range
                    .line_calculators
                    .iter()
                    .map(|v| v.calculate_y_value(x))
                    .collect();
                ys.sort_unstable();

                for two_ys in ys.chunks_exact(2) {
                    let mut y1 = *unsafe { two_ys.get_unchecked(0) };
                    let mut y2 = *unsafe { two_ys.get_unchecked(1) };

                    if y1 > y2 {
                        (y1, y2) = (y2, y1);
                    }
                    for y in y1..=y2 {
                        canvas[(x as usize, y as usize)] = *color;
                    }
                }
            }
        }
    }
}
