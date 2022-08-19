use crate::{
    drawin::drawable::Drawable,
    geometry::{discrete_line::DiscreteLine, discrete_point::DiscretePoint, rect_size::RectSize},
    wavefront::wavefront_obj::WavefrontObj,
};

impl Drawable for WavefrontObj {
    fn draw(
        &self,
        canvas: &mut crate::drawin::draw_buffer::DrawBuffer,
        color: &crate::drawin::color::Color,
    ) {
        let RectSize { width, height } = canvas.get_size();

        let (w_f32, h_f32) = ((width - 1) as f32, (height - 1) as f32);

        for i in 0..self.faces.len() {
            let face = self.faces[i];
            for j in 0..3_usize {
                let v0 = self.vertices[face.0[j]];
                let v1 = self.vertices[face.0[(j + 1) % 3]];
                let x0 = ((*v0.x() + 1.0) * w_f32 / 2.0) as isize;
                let y0 = ((*v0.y() + 1.0) * h_f32 / 2.0) as isize;
                let x1 = ((*v1.x() + 1.0) * w_f32 / 2.0) as isize;
                let y1 = ((*v1.y() + 1.0) * h_f32 / 2.0) as isize;

                let begin = DiscretePoint { x: x0, y: y0 };
                let end = DiscretePoint { x: x1, y: y1 };
                let line = DiscreteLine { begin, end };
                line.draw(canvas, color);
            }
        }
    }
}
