use crate::{geometry::primitives::discrete_polygon::DiscretePolygon, drawin::drawable::Drawable};

pub type DiscreteTriangle = DiscretePolygon<3>;

impl Drawable for DiscreteTriangle {
    fn draw(&self, canvas: &mut crate::drawin::draw_buffer::DrawBuffer, color: &crate::drawin::color::Color) {
        let x_begin = self.0;
    }
}