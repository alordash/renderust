use crate::{
    drawin::{color::Color, drawable::Drawable},
    geometry::{discrete_line::DiscreteLine, discrete_point::DiscretePoint},
};

impl Drawable for DiscreteLine {
    fn draw(&self, canvas: &mut crate::drawin::draw_buffer::DrawBuffer, color: &Color) {
        let mut p1 = self.begin;
        let mut p2 = self.end;

        // if p2.x < p1.x {
        //     (p1, p2) = (p2, p1);
        // }

        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        let k = dy as f32 / dx as f32;
        let mut angle_bigger_45_deg = false;

        // if k.abs() > 1.0 {
        //     angle_bigger_45_deg = true;
        //     (p1.x, p1.y) = (p1.y, p1.y);
        //     (p2.x, p2.y) = (p2.y, p2.x);
        // }

        let color = Color::random();

        for x in p1.x..p2.x {
            let y = (k * (x - p1.x) as f32) as isize + p1.y;
            canvas[(x as usize, y as usize)] = color;
        }
    }
}
