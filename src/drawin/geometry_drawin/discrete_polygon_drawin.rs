use crate::{drawin::drawable::Drawable, geometry::primitives::discrete_polygon::DiscretePolygon};



impl<const N: usize> Drawable for DiscretePolygon<N> {
    default fn draw(&self, canvas: &mut crate::drawin::draw_buffer::DrawBuffer, color: &crate::drawin::color::Color) {
        for p in self.points.iter() {
            p.draw(canvas, color);
        }
    }
}