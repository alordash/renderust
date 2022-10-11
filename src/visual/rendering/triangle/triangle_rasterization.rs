use glam::Vec3;
use image::{DynamicImage, GenericImage};

use crate::{
    math::{
        geometry::primitives::polygon::Polygon,
        interpolation::Interpolator,
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
    normal_map: Option<&PlaneBuffer<Vec3>>,
    light_dir: Vec3,
    look_dir: Vec3,
    color: Option<&Color>,
) {
    let points = polygon.get_points();
    let mut points_sorted_by_x = points.clone();
    points_sorted_by_x.sort_unstable_by(|a, b| a.x.cmp(&b.x));
    let (texture_width, texture_height) = texture.dimensions();

    let (l_p, m_p, r_p) = unsafe {
        (
            *points_sorted_by_x.get_unchecked(0),
            *points_sorted_by_x.get_unchecked(1),
            *points_sorted_by_x.get_unchecked(2),
        )
    };

    let (l_v, m_v, r_v) = (
        PolygonInterpolationValues::from(l_p),
        PolygonInterpolationValues::from(m_p),
        PolygonInterpolationValues::from(r_p),
    );

    let (l_calc, long_calc, r_calc) = (
        Interpolator::from((l_p.x, m_p.x)),
        Interpolator::from((l_p.x, r_p.x)),
        Interpolator::from((m_p.x, r_p.x)),
    );

    let d_long_v = r_v - l_v;
    let d_x = r_p.x - l_p.x;

    let mut filler = |short_calc: Interpolator<i32>,
                      v_start: PolygonInterpolationValues,
                      v_end: PolygonInterpolationValues| {
        let d_interp = v_end - v_start;
        let range = short_calc.get_interpolation_range();
        let range_start = range.start;
        for x in range {
            let mut v1 = short_calc.interpolate(x, d_interp, v_start);
            v1.color = v_start.color.interpolate(
                v_end.color,
                (x - range_start) as i32,
                (*short_calc.get_diff()) as i32,
            );
            let mut v2 = long_calc.interpolate(x, d_long_v, l_v);
            v2.color = l_v
                .color
                .interpolate(r_v.color, (x - l_p.x) as i32, d_x as i32);

            if v1.y > v2.y {
                (v1, v2) = (v2, v1);
            }

            let (y1, y2) = (v1.y, v2.y);
            let dy = y2 - y1;

            let local_calc = Interpolator::new(y1, y2);
            let local_d_v = v2 - v1;

            for y in y1..y2 {
                let p = (x as usize, y as usize);

                let local_v = local_calc.interpolate(y, local_d_v, v1);

                let PolygonInterpolationValues {
                    y,
                    z_depth,
                    uv,
                    mut normal,
                    has_color,
                    ..
                } = local_v;

                let z_val = &mut canvas.get_z_buffer_mut()[p];
                if *z_val > z_depth {
                    continue;
                }

                let (uvx, uvy) = (
                    (uv.x * texture_width as f32) as u32,
                    (uv.y * texture_height as f32) as u32,
                );

                if let Some(normal_map) = normal_map {
                    let nm = normal_map[(uvx as usize, uvy as usize)];
                    normal = nm;
                }

                let visibility = look_dir.dot(normal);
                if visibility < 0.0 {
                    continue;
                }

                let intensity = light_dir.dot(normal).max(0.0);
                let new_color = if has_color {
                    v1.color.interpolate(v2.color, (y - y1) as i32, dy as i32)
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

    filler(l_calc, l_v, m_v);
    filler(r_calc, m_v, r_v);
}
