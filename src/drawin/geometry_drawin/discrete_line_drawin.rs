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
        let mut k = dy as f32 / dx as f32;
        let mut angle_bigger_45_deg = false;

        if k.abs() > 1.0 {
            angle_bigger_45_deg = true;
            (p1.x, p1.y) = (p1.y, p1.x);
            (p2.x, p2.y) = (p2.y, p2.x);
            k = 1.0 / k;
            (dx, dy) = (dy, dx);
        }

        if p2.x < p1.x {
            (p1, p2) = (p2, p1);
        }

        for x in p1.x..p2.x {
            let y = (k * (x - p1.x) as f32) as isize + p1.y;
            let (x0, y0) = if angle_bigger_45_deg { (y, x) } else { (x, y) };
            canvas[(x0 as usize, y0 as usize)] = *color;
        }
    }
}
