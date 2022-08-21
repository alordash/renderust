use crate::{
    drawin::drawable::Drawable,
    geometry::primitives::{
        discrete_line::DiscreteLine, polygons::discrete_triangle::DiscreteTriangle,
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

        let mut long_line_iter = DiscreteLine {
            begin: *left_point,
            end: *right_point,
        }
        .into_iter();
        let mut left_line_iter = DiscreteLine {
            begin: *left_point,
            end: *middle_point,
        }
        .into_iter();
        let mut right_line_iter = DiscreteLine {
            begin: *middle_point,
            end: *right_point,
        }
        .into_iter();

        let mut short_line_iter = &mut left_line_iter;
        let mut short_line_left_iters = short_line_iter.get_iterations_count();

        for x in left_point.x..right_point.x {
            let p1 = long_line_iter.next().unwrap();
            let p2 = short_line_iter.next().unwrap();

            // draw..
            let y_min = p1.y.min(p2.y);
            let y_max = p1.y.max(p2.y);

            for y in y_min..y_max {
                canvas[(x as usize, y as usize)] = *color;
            }

            if short_line_left_iters > 0 {
                short_line_left_iters -= 1;
                if short_line_left_iters == 0 {
                    short_line_iter = &mut right_line_iter;
                }
            }
        }
    }
}
