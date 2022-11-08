use glam::{Mat3A, Vec3A};
use image::{DynamicImage, GenericImageView};

use crate::{
    math::{
        geometry::apply_transform_matrix::vertex_apply_transform_matrix,
        interpolation::Interpolator,
    },
    plane_buffer::plane_buffer::PlaneBuffer,
    visual::{
        color::color::Color,
        drawing_buffer::DrawingBuffer,
        rendering::{
            light_source::{LightSource, LightSourceKind},
            triangle::interpolation_values::InterpolationValues,
        },
        vertex::Vertex,
    },
};

pub fn render_triangle_mesh(
    vertices: &[Vertex; 3],
    canvas: &mut DrawingBuffer,
    texture: &DynamicImage,
    lights: &mut Vec<LightSource>,
    normal_map: Option<&PlaneBuffer<Vec3A>>,
    spec_map: Option<&DynamicImage>,
    glow_map: Option<&DynamicImage>,
) {
    let mut vertices_sorted_by_x = vertices.clone();
    vertices_sorted_by_x.sort_unstable_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    let (texture_width, texture_height) = texture.dimensions();

    let (nm_width, nm_height) = (
        normal_map.map(PlaneBuffer::get_width).map(|w| w as u32),
        normal_map.map(PlaneBuffer::get_height).map(|h| h as u32),
    );

    let (sp_width, sp_height) = (
        spec_map.map(GenericImageView::width),
        spec_map.map(GenericImageView::height),
    );

    let (gw_width, gw_height) = (
        glow_map.map(GenericImageView::width),
        glow_map.map(GenericImageView::height),
    );

    let (l_p, m_p, r_p) = unsafe {
        (
            *vertices_sorted_by_x.get_unchecked(0),
            *vertices_sorted_by_x.get_unchecked(1),
            *vertices_sorted_by_x.get_unchecked(2),
        )
    };

    let (l_v, m_v, r_v) = (
        InterpolationValues::from(l_p),
        InterpolationValues::from(m_p),
        InterpolationValues::from(r_p),
    );

    let (l_calc, long_calc, r_calc) = (
        Interpolator::from((l_p.x as i32, m_p.x as i32)),
        Interpolator::from((l_p.x as i32, r_p.x as i32)),
        Interpolator::from((m_p.x as i32, r_p.x as i32)),
    );

    let d_long_v = r_v - l_v;

    let A = Mat3A::from_cols(
        m_p.screen_pos - l_p.screen_pos,
        r_p.screen_pos - l_p.screen_pos,
        Vec3A::ZERO,
    );

    let (l_uv, m_uv, r_uv) = (l_p.uv, m_p.uv, r_p.uv);

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

            let (y1, y2) = (v1.y as i32, v2.y as i32);

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

                if let Some(normal_map) = normal_map {
                    let (nm_width, nm_height) = (nm_width.unwrap(), nm_height.unwrap());
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

                let mut glow = Vec3A::ZERO;

                if let Some(glow_map) = glow_map {
                    let (gw_width, gw_height) = (gw_width.unwrap(), gw_height.unwrap());
                    let (gwuvx, gwuvy) = (
                        ((uv.x * gw_width as f32) as u32).min(gw_width - 1),
                        ((uv.y * gw_height as f32) as u32).min(gw_height - 1),
                    );

                    let rgba = glow_map.get_pixel(gwuvx, gwuvy).0;
                    glow = Vec3A::new(rgba[0] as f32, rgba[1] as f32, rgba[2] as f32) / 128.0;
                }

                let mut intensities = Vec3A::ZERO;

                for light in lights.iter_mut() {
                    match &mut light.kind {
                        LightSourceKind::Linear {
                            dir,
                            shadow_buffer,
                            transform_matrix,
                        } => {
                            let mut self_shadow = 1.0;
                            if let Some(shadow_buffer) = shadow_buffer {
                                let transform_matrix = transform_matrix.unwrap();
                                let shadow_coord = vertex_apply_transform_matrix(
                                    Vec3A::new(x as f32, y as f32, z_depth),
                                    transform_matrix,
                                );
                                let shadow_2d_coord =
                                    (shadow_coord.x as usize, shadow_coord.y as usize);
                                if shadow_buffer.contains(shadow_2d_coord.0, shadow_2d_coord.1) {
                                    let shadowed =
                                        (shadow_coord.z + 4.0) < shadow_buffer[shadow_2d_coord];

                                    self_shadow = 0.0 + 1.0 * (if shadowed { 0.0 } else { 1.0 });
                                }
                            }

                            let mut spec = Vec3A::ZERO;

                            if let Some(spec_map) = spec_map {
                                let (sp_width, sp_height) = (sp_width.unwrap(), sp_height.unwrap());
                                let (spuvx, spuvy) = (
                                    ((uv.x * sp_width as f32) as u32).min(sp_width - 1),
                                    ((uv.y * sp_height as f32) as u32).min(sp_height - 1),
                                );

                                let reflection = normal * (normal.dot(*dir) * 2.0) - *dir;

                                let spec_coeff =
                                    (255.0 - spec_map.get_pixel(spuvx, spuvy).0[2] as f32) / 32.0;

                                let reflected =
                                    (reflection.normalize().z + 0.05).max(0.0).powf(spec_coeff);
                                spec = Vec3A::ONE * reflected * light.spectrum;
                            }

                            intensities += (light.spectrum
                                * dir.dot(normal).max(0.0).powf(light.concentration)
                                + 0.95 * spec)
                                * self_shadow;
                        }
                        LightSourceKind::Ambient => intensities += light.spectrum,
                    }
                }

                intensities += glow;

                let new_color =
                    Color::from(texture.get_pixel(uvx, uvy)).apply_intensity(intensities);
                *z_val = z_depth;
                canvas[p] = new_color;
            }
        }
    };

    filler(l_calc, l_v, m_v);
    filler(r_calc, m_v, r_v);
}
