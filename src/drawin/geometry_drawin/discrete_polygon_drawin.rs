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

            println!("found {} suitable lines", suitable_lines.len());
            // let l1 = unsafe { suitable_lines.get_unchecked(0) };
            // let l2 = unsafe { suitable_lines.get_unchecked(1) };
            for x in range {
                // let mut y1 = suitable_lines[0].next().unwrap().y;
                // let mut y2 = suitable_lines[1].next().unwrap().y;
                suitable_lines
                    .sort_unstable_by(|a, b| a.calculate_y_value(x).cmp(&b.calculate_y_value(x)));
                for two_calcs in suitable_lines.windows(2) {
                    let calc1 = unsafe { two_calcs.get_unchecked(0) };
                    let calc2 = unsafe { two_calcs.get_unchecked(1) };
                    let mut y1 = calc1.calculate_y_value(x);
                    let mut y2 = calc2.calculate_y_value(x);
                    if y1 > y2 {
                        (y1, y2) = (y2, y1);
                    }
                    for y in y1..y2 {
                        canvas[(x as usize, y as usize)] = *color;
                    }
                }
            }
        }
    }
}
