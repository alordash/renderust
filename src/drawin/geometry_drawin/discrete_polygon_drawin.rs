use crate::{
    discretization::geometry_discretization::discrete_line_iterator::DiscreteLineIterator,
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
        let mut x_sorted_points = self.points.clone();
        // x_sorted_points.sort_unstable_by(|a, b| a.x.cmp(&b.x));
        let mut lines: Vec<_> = x_sorted_points
            .windows(2)
            .map(|two_points| unsafe {
                DiscreteLine {
                    begin: *two_points.get_unchecked(0),
                    end: *two_points.get_unchecked(1),
                }
            })
            .collect();
        lines.push(DiscreteLine { begin: x_sorted_points[x_sorted_points.len() - 1], end: x_sorted_points[0] });
        println!("lines: {:?}", lines);
        lines.iter().for_each(|l| l.draw(canvas, color));
    }
}
