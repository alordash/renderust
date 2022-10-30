use glam::{Mat3A, Vec3A};
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
    normal_map: &PlaneBuffer<Vec3A>,
    light_dir: Vec3A,
    use_normal_map: bool,
    z_buffer_size: f32,
) {
    let points = polygon.get_points();
    let mut points_sorted_by_x = points.clone();
    points_sorted_by_x.sort_unstable_by(|a, b| a.x.cmp(&b.x));
    let (texture_width, texture_height) = texture.dimensions();

    let (nm_width, nm_height) = (
        normal_map.get_width() as u32,
        normal_map.get_height() as u32,
    );

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

    let A = Mat3A::from_cols(
        Vec3A::from((
            (m_p.coords - l_p.coords).as_vec2(),
            (m_p.get_z_depth() - l_p.get_z_depth()) as f32,
        )),
        Vec3A::from((
            (r_p.coords - l_p.coords).as_vec2(),
            (r_p.get_z_depth() - l_p.get_z_depth()) as f32,
        )),
        Vec3A::ZERO,
    );

    let (l_uv, m_uv, r_uv) = (*l_p.get_uv(), *m_p.get_uv(), *r_p.get_uv());

    let I = Vec3A::new(m_uv.x - l_uv.x, r_uv.x - l_uv.x, 0.0);
    let J = Vec3A::new(m_uv.y - l_uv.y, r_uv.y - l_uv.y, 0.0);

    let mut filler = |short_calc: Interpolator<i32>,
                      v_start: InterpolationValues,
                      v_end: InterpolationValues| {
        let d_interp = v_end - v_start;
        let range = short_calc.get_interpolation_range();

        for x in range {
            let mut v1 = short_calc.interpolate(x, d_interp, v_start);
            let mut v2 = long_calc.interpolate(x, d_long_v, l_v);

            if v1.y > v2.y {
                (v1, v2) = (v2, v1);
            }

            let (y1, y2) = (v1.y, v2.y);

            let local_calc = Interpolator::new(y1, y2);
            let local_d_v = v2 - v1;

            for y in y1..y2 {
                let p = (x as usize, y as usize);
                if !canvas.contains(p.0, p.1) {
                    continue;
                }

                let local_v = local_calc.interpolate(y, local_d_v, v1);

                let InterpolationValues {
                    z_depth,
                    uv,
                    mut normal,
                    ..
                } = local_v;

                let z_val = &mut canvas.get_z_buffer_mut()[p];
                if *z_val > z_depth {
                    continue;
                }

                let (uvx, uvy) = (
                    ((uv.x * texture_width as f32) as u32).min(texture_width - 1),
                    ((uv.y * texture_height as f32) as u32).min(texture_height - 1),
                );

                if use_normal_map {
                    let (nuvx, nuvy) = (
                        ((uv.x * nm_width as f32) as u32).min(nm_width - 1),
                        ((uv.y * nm_height as f32) as u32).min(nm_height - 1),
                    );

                    let mut AI = A.transpose();
                    *AI.col_mut(2) = normal;
                    AI = AI.inverse();

                    let i = AI * I;
                    let j = AI * J;

                    let B = Mat3A::from_cols(i.normalize(), j.normalize(), normal);

                    let nm = normal_map[(nuvx as usize, nuvy as usize)];
                    normal = (B * nm).normalize();
                }

                let mut intensity = light_dir.dot(normal).max(0.0);

                intensity = if intensity < 0.33 {
                    0.0
                } else if intensity < 0.66 {
                    0.33
                } else if intensity < 1.0 {
                    0.66
                } else {
                    1.0
                };

                let new_color = Color::from(texture.get_pixel(uvx, uvy)) * intensity;
                *z_val = z_depth;
                let shade = (z_depth * (u8::MAX) as f32 / z_buffer_size) as u8;
                canvas[p] = Color::from_rgb(shade, shade, shade);
            }
        }
    };

    filler(l_calc, l_v, m_v);
    filler(r_calc, m_v, r_v);
}
