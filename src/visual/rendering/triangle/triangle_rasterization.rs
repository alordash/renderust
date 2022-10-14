use glam::{Mat3, Vec3, Vec3Swizzles};
use image::{DynamicImage, GenericImage};

use crate::{
    math::{geometry::primitives::polygon::Polygon, interpolation::Interpolator},
    plane_buffer::plane_buffer::PlaneBuffer,
    visual::{
        color::color::Color, drawing_buffer::DrawingBuffer,
        rendering::interpolation_values::InterpolationValues,
    },
};

pub fn fill_triangle(
    polygon: &Polygon<3>,
    canvas: &mut DrawingBuffer,
    texture: &DynamicImage,
    maybe_normal_map: Option<&PlaneBuffer<Vec3>>,
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
        InterpolationValues::from(l_p),
        InterpolationValues::from(m_p),
        InterpolationValues::from(r_p),
    );

    let (l_calc, long_calc, r_calc) = (
        Interpolator::from((l_p.x, m_p.x)),
        Interpolator::from((l_p.x, r_p.x)),
        Interpolator::from((m_p.x, r_p.x)),
    );

    let d_long_v = r_v - l_v;
    let d_x = r_p.x - l_p.x;

    let mut A = Mat3::from_cols(
        Vec3::from((
            (m_p.coords - l_p.coords).as_vec2(),
            (m_p.get_z_depth() - l_p.get_z_depth()) as f32,
        )),
        Vec3::from((
            (r_p.coords - l_p.coords).as_vec2(),
            (r_p.get_z_depth() - l_p.get_z_depth()) as f32,
        )),
        Vec3::ZERO,
    );

    let (l_uv, m_uv, r_uv) = (*l_p.get_uv(), *m_p.get_uv(), *r_p.get_uv());

    let I = Vec3::new(m_uv.x - l_uv.x, r_uv.x - l_uv.x, 0.0);
    let J = Vec3::new(m_uv.y - l_uv.y, r_uv.y - l_uv.y, 0.0);

    let mut filler = |short_calc: Interpolator<i32>,
                      v_start: InterpolationValues,
                      v_end: InterpolationValues| {
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

                let InterpolationValues {
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

                if let Some(normal_map) = maybe_normal_map {
                    let (nuvx, nuvy) = (
                        (uv.x * normal_map.get_width() as f32) as u32,
                        (uv.y * normal_map.get_height() as f32) as u32,
                    );

                    *A.col_mut(2) = normal;
                    let AI = A.transpose().inverse();
                    let i = AI * I;
                    let j = AI * J;

                    let B = Mat3::from_cols(i.normalize(), j.normalize(), normal);

                    let nm = normal_map[(nuvx as usize, nuvy as usize)];
                    normal = (B * nm).normalize();
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
