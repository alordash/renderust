use crate::{drawin::{drawable::Drawable, color::Color, draw_buffer::DrawBuffer}, geometry::primitives::point::Point};

impl Drawable for Point {
    fn draw(
        &self,
        canvas: &mut DrawBuffer,
        color: Option<&Color>,
    ) {
        let size = 9;
        let x_start = self.x - size / 2;
        let y_start = self.y - size / 2;
        let color = color.unwrap_or(&self.color);
        for x in x_start..x_start + size {
            for y in y_start..y_start + size {
                canvas[(x as usize, y as usize)] = *color;
            }
        }
    }
}
