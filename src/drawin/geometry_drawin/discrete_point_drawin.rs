use crate::{drawin::drawable::Drawable, geometry::primitives::discrete_point::DiscretePoint};

impl Drawable for DiscretePoint {
    fn draw(
        &self,
        canvas: &mut crate::drawin::draw_buffer::DrawBuffer,
        color: &crate::drawin::color::Color,
    ) {
        let size = 9;
        let x_start = self.x - size / 2;
        let y_start = self.y - size / 2;
        for x in x_start..x_start + size {
            for y in y_start..y_start + size {
                canvas[(x as usize, y as usize)] = *color;
            }
        }
    }
}
