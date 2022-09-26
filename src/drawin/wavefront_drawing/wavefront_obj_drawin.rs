use crate::{
    drawin::{color::Color, drawable::Drawable},
    geometry::{
        math_vectors::{vec3f::Vec3f, Vec3},
        primitives::discrete_line::DiscreteLine,
        primitives::{
            discrete_point::DiscretePoint, polygons::discrete_triangle::DiscreteTriangle,
        },
        rect_size::RectSize,
    },
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
                let x0 = ((v0.x() + 1.0) * w_f32 / 2.0) as isize;
                let y0 = ((v0.y() + 1.0) * h_f32 / 2.0) as isize;
                let x1 = ((v1.x() + 1.0) * w_f32 / 2.0) as isize;
                let y1 = ((v1.y() + 1.0) * h_f32 / 2.0) as isize;

                let begin = DiscretePoint { x: x0, y: y0 };
                let end = DiscretePoint { x: x1, y: y1 };
                let line = DiscreteLine { begin, end };
                line.draw(canvas, color);
            }
        }
    }

    fn fill(
        &self,
        canvas: &mut crate::drawin::draw_buffer::DrawBuffer,
        color: &crate::drawin::color::Color,
    ) {
        let RectSize { width, height } = canvas.get_size();
        let light_dir = Vec3::<f32>([0.0, 0.0, -1.0]);

        let (w_f32, h_f32) = ((width - 1) as f32, (height - 1) as f32);

        for i in 0..self.faces.len() {
            let face = self.faces[i];
            let mut world_coords = [Vec3f::default(); 3];
            let mut screen_coords = [DiscretePoint { x: 0, y: 0 }; 3];
            for j in 0..3_usize {
                let v0 = self.vertices[face.0[j]];
                let x0 = ((v0.x() + 1.0) * w_f32 / 2.0) as isize;
                let y0 = ((v0.y() + 1.0) * h_f32 / 2.0) as isize;
                world_coords[j] = v0;
                screen_coords[j] = DiscretePoint { x: x0, y: y0 };
            }
            let first_edge = world_coords[2] - world_coords[0];
            let second_edge = world_coords[1] - world_coords[0];

            let mut normal = first_edge.cross_product(second_edge);
            normal.normalize();

            let intensity = normal.dot_product(light_dir);

            if intensity > 0.0 {
                let triangle = DiscreteTriangle {
                    points: screen_coords,
                };
                triangle.draw(canvas, &(*color * intensity));
            }
        }
    }
}
