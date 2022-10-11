use std::os::windows;

use image::{DynamicImage, GenericImage};
use nalgebra::Matrix3;

use crate::{
    math::{
        geometry::primitives::polygon::Polygon,
        interpolation::Interpolator,
        vector::{common_vectors::vec3f::Vec3f, linear_algebra::LinAlgOperations},
    },
    plane_buffer::plane_buffer::PlaneBuffer,
    visual::{
        color::color::Color, drawing_buffer::DrawingBuffer,
        rendering::polygon::polygon_interpolation_values::PolygonInterpolationValues,
    },
};

pub fn fill_triangle(
    polygon: &Polygon<3>,
    canvas: &mut DrawingBuffer,
    texture: &DynamicImage,
    normal_map: Option<&PlaneBuffer<Vec3f>>,
    light_dir: Vec3f,
    look_dir: Vec3f,
    color: Option<&Color>,
) {
    let points = polygon.get_points();
    let mut points_sorted_by_x = points.clone();
    points_sorted_by_x.sort_unstable_by(|a, b| a.x().cmp(&b.x()));
    let (texture_width, texture_height) = texture.dimensions();

    let (left_point, middle_point, right_point) = unsafe {
        (
            *points_sorted_by_x.get_unchecked(0),
            *points_sorted_by_x.get_unchecked(1),
            *points_sorted_by_x.get_unchecked(2),
        )
    };

    let (l_interp, m_interp, r_interp) = (
        PolygonInterpolationValues::from(left_point),
        PolygonInterpolationValues::from(middle_point),
        PolygonInterpolationValues::from(right_point),
    );

    let (left_interpolator, long_interpolator, right_interpolator) = (
        Interpolator::from((left_point.x(), middle_point.x())),
        Interpolator::from((left_point.x(), right_point.x())),
        Interpolator::from((middle_point.x(), right_point.x())),
    );

    let d_long_interpolation = r_interp - l_interp;
    let dx = right_point.x() - left_point.x();

    let mut filling_fn = |short_interpolator: Interpolator<isize>,
                          interpolation_start: PolygonInterpolationValues,
                          interpolation_end: PolygonInterpolationValues| {
        let d_interp = interpolation_end - interpolation_start;
        let range = short_interpolator.get_interpolation_range();
        let range_start = range.start;
        for x in range {
            let mut interp1 = short_interpolator.interpolate(x, d_interp, interpolation_start);
            interp1.color = interpolation_start.color.interpolate(
                interpolation_end.color,
                (x - range_start) as i32,
                (*short_interpolator.get_diff()) as i32,
            );
            let mut interp2 = long_interpolator.interpolate(x, d_long_interpolation, l_interp);
            interp2.color =
                l_interp
                    .color
                    .interpolate(r_interp.color, (x - left_point.x()) as i32, dx as i32);

            if interp1.y > interp2.y {
                (interp1, interp2) = (interp2, interp1);
            }

            let (y1, y2) = (interp1.y, interp2.y);
            let dy = y2 - y1;

            let local_interpolator = Interpolator::new(y1, y2);
            let local_d_interp = interp2 - interp1;

            for y in y1..y2 {
                let p = (x as usize, y as usize);

                let local_interp = local_interpolator.interpolate(y, local_d_interp, interp1);

                let PolygonInterpolationValues {
                    y,
                    z_depth,
                    uv,
                    mut normal,
                    has_color,
                    ..
                } = local_interp;

                let z_val = &mut canvas.get_z_buffer_mut()[p];
                if *z_val > z_depth {
                    continue;
                }

                let (uvx, uvy) = (
                    (uv.x() * texture_width as f32) as u32,
                    (uv.y() * texture_height as f32) as u32,
                );

                if let Some(normal_map) = normal_map {
                    let nm = normal_map[(uvx as usize, uvy as usize)];
                    normal = nm;
                }

                let visibility = look_dir.dot_product(normal);
                if visibility < 0.0 {
                    continue;
                }

                let intensity = light_dir.dot_product(normal).max(0.0);
                let new_color = if has_color {
                    interp1
                        .color
                        .interpolate(interp2.color, (y - y1) as i32, dy as i32)
                } else {
                    color
                        .map(|c| *c)
                        .unwrap_or(Color::from(texture.get_pixel(uvx, uvy)))
                } * intensity;
                *z_val = z_depth;
                canvas[p] = new_color;
            }
        }
    };

    filling_fn(left_interpolator, l_interp, m_interp);
    filling_fn(right_interpolator, m_interp, r_interp);
}
