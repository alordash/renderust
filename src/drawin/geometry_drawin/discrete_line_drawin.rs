use crate::{
    drawin::{color::Color, drawable::Drawable},
    geometry::{discrete_line::DiscreteLine, discrete_point::DiscretePoint},
};

impl Drawable for DiscreteLine {
    fn draw(&self, canvas: &mut crate::drawin::draw_buffer::DrawBuffer, color: &Color) {\
        let DiscretePoint { x: mut x0, y: mut y0 } = self.begin;
        let DiscretePoint { x: mut x1, y: mut y1 } = self.end;

        if x1 < x0 {
            (x1, x0) = (x0, x1);
            (y1, y0) = (y0, y1);
        }

        let width = x1 - x0;
        let height = y1 - y0;

        for i in 0..width {
            let x = x0 + i;
            let y = y0 + height * i / width;
            canvas[(x as usize, y as usize)] = color.clone();
        }
    }
}
