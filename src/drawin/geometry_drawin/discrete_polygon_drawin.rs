use crate::{
    discretization::geometry_discretization::{
        discrete_line_iterator::DiscreteLineIterator,
        discrete_line_x_axis_calculator::DiscreteLineXAxisCalculator,
    },
    drawin::drawable::Drawable,
    geometry::primitives::{discrete_line::DiscreteLine, discrete_polygon::DiscretePolygon},
};

struct FillingRegion<'a> {
    lineIterators: Vec<&'a mut DiscreteLineIterator>,
}

impl<const N: usize> Drawable for DiscretePolygon<N> {
    default fn draw(
        &self,
        canvas: &mut crate::drawin::draw_buffer::DrawBuffer,
        color: &crate::drawin::color::Color,
    ) {
        let mut lines: Vec<_> = self
            .points
            .windows(2)
            .map(|two_points| unsafe {
                DiscreteLine {
                    begin: *two_points.get_unchecked(0),
                    end: *two_points.get_unchecked(1),
                }
            })
            .collect();
        lines.push(DiscreteLine {
            begin: self.points[self.points.len() - 1],
            end: self.points[0],
        });
        // println!("lines: {:?}", lines);
        lines.iter().for_each(|l| l.draw(canvas, color));
        lines.iter_mut().for_each(|l| l.order_by_x());

        let line_calculators: Vec<_> = lines
            .iter()
            .map(|l| DiscreteLineXAxisCalculator::from(*l))
            .collect();

        let mut x_sorted_points = self.points.clone();
        x_sorted_points.sort_unstable_by(|a, b| a.x.cmp(&b.x));
        let ranges = x_sorted_points.windows(2).map(|two_points| unsafe {
            two_points.get_unchecked(0).x..two_points.get_unchecked(1).x
        });

        for range in ranges {
            let mut suitable_lines = line_calculators
                .iter()
                .enumerate()
                .filter(|(i, _)| {
                    let l = lines[*i];
                    l.begin.x <= range.start && range.end <= l.end.x
                })
                .map(|(_, b)| b)
                .collect::<Vec<_>>();

            // let l1 = unsafe { suitable_lines.get_unchecked(0) };
            // let l2 = unsafe { suitable_lines.get_unchecked(1) };
            for x in range {
                let mut ys: Vec<isize> = suitable_lines
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
