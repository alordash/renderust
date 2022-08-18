use crate::{
    drawin::{color::Color, drawable::Drawable},
    geometry::{discrete_line::DiscreteLine, discrete_point::DiscretePoint},
};

impl Drawable for DiscreteLine {
    fn draw(&self, canvas: &mut crate::drawin::draw_buffer::DrawBuffer, color: &Color) {
        let mut p1 = self.begin;
        let mut p2 = self.end;

        let mut dx = p2.x - p1.x;
        let mut dy = p2.y - p1.y;
        let mut angle_bigger_45_deg = false;

        if dy.abs() > dx.abs() {
            angle_bigger_45_deg = true;
            (p1.x, p1.y) = (p1.y, p1.x);
            (p2.x, p2.y) = (p2.y, p2.x);
        }

        if p2.x < p1.x {
            (p1, p2) = (p2, p1);
        }

        dx = p2.x - p1.x;
        dy = p2.y - p1.y;

        let dy_error = dy.abs() * 2;
        let mut y_error = 0;
        let mut y = p1.y as usize;

        let iter_range = p1.x as usize..p2.x as usize;
        for x in iter_range {
            // This condition can be optimized
            // by taking it out of loop
            if angle_bigger_45_deg {
                canvas[(y, x)] = *color;
            } else {
                canvas[(x, y)] = *color;
            }

            y_error += dy_error;
            if y_error > dx {
                // This condition can be optimized
                // by taking it out of loop
                if p2.y > p1.y {
                    y += 1;
                } else {
                    y -= 1;
                }
                y_error -= dx * 2;
            }
        }
    }
}
