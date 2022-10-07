use image::{DynamicImage, GenericImage};

use crate::{
    discretization::geometry_discretization::line_x_axis_calculator::LineXAxisCalculator,
    drawin::{
        color::Color, draw_buffer::DrawBuffer, geometry_drawin::polygon_drawin::PolygonFillingRange,
    },
    geometry::{primitives::{line::Line, polygon::Polygon}, math_vectors::vec3f::Vec3f},
};

pub type Triangle = Polygon<3>;

impl Triangle {
    pub fn fill_with_texture(
        &self,
        canvas: &mut DrawBuffer,
        texture: &DynamicImage,
        light_dir: Vec3f,
    ) {
        let mut lines = self.get_perimeter_lines();
        lines.iter_mut().for_each(Line::order_by_x);

        let line_calculators: Vec<LineXAxisCalculator> =
            lines.into_iter().map(LineXAxisCalculator::from).collect();

        let mut x_sorted_points = self.points.clone();
        x_sorted_points.sort_unstable_by(|a, b| a.x().cmp(&b.x()));
        let polygon_filling_ranges = x_sorted_points.windows(2).map(|two_points| unsafe {
            let range = two_points.get_unchecked(0).x()..two_points.get_unchecked(1).x();
            let suitable_line_calculators: Vec<&LineXAxisCalculator> = line_calculators
                .iter()
                .filter(|lc| {
                    let lc_range = lc.get_x_calculation_range();
                    lc_range.start <= range.start && range.end <= lc_range.end
                })
                .collect();
            PolygonFillingRange {
                line_calculators: suitable_line_calculators,
                range,
            }
        });

        let (width, height) = texture.dimensions();

        for polygon_filling_range in polygon_filling_ranges.into_iter() {
            for x in polygon_filling_range.range {
                let mut yz_uv_nms: Vec<_> = polygon_filling_range
                    .line_calculators
                    .iter()
                    .map(|v| v.calculate_y_and_z_and_uv_and_normal_value(x))
                    .collect();
                yz_uv_nms.sort_unstable_by(|a, b| a.0.cmp(&b.0));

                for two_ys in yz_uv_nms.chunks_exact(2) {
                    let mut yz_uv_nm1 = *unsafe { two_ys.get_unchecked(0) };
                    let mut yz_uv_nm2 = *unsafe { two_ys.get_unchecked(1) };

                    if yz_uv_nm1.0 > yz_uv_nm2.0 {
                        (yz_uv_nm1, yz_uv_nm2) = (yz_uv_nm2, yz_uv_nm1);
                    }
                    let ((y1, z1, uv1, nm1), (y2, z2, uv2, nm2)) = (
                        (yz_uv_nm1.0, yz_uv_nm1.1, yz_uv_nm1.2, yz_uv_nm1.3),
                        (yz_uv_nm2.0, yz_uv_nm2.1, yz_uv_nm2.2, yz_uv_nm2.3),
                    );
                    let d = y2 - y1;
                    let inv_d = 1.0 / d as f32;
                    let dz = z2 - z1;
                    let duv = uv2 - uv1;
                    let dnm = nm2 - nm1;

                    for y in y1..y2 {
                        let dy = y - y1;
                        let z = dz * dy / d + z1;
                        let uv = duv * dy as f32 * inv_d + uv1;
                        let uvx = (uv.0[0] * width as f32) as u32;
                        let uvy = (uv.0[1] * height as f32) as u32;
                        let nm = dnm * dy as f32 * inv_d + nm1;
                        let intensity = light_dir.dot_product(nm).max(0.0);
                        let new_color = Color::from(texture.get_pixel(uvx, uvy)) * intensity;
                        let p = (x as usize, y as usize);
                        if canvas.1[p] < z {
                            canvas.1[p] = z;
                            canvas[p] = new_color;
                        }
                    }
                }
            }
        }
    }
}
