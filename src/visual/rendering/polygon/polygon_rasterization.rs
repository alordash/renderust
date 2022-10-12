use glam::Vec3;
use image::{DynamicImage, GenericImage};

use crate::{
    math::{
        geometry::primitives::{line::Line, polygon::Polygon},
        interpolation::Interpolator,
    },
    plane_buffer::plane_buffer::PlaneBuffer,
    visual::{
        color::color::Color, drawing_buffer::DrawingBuffer,
        rendering::{line::line_rasterization::draw_line, interpolation_values::InterpolationValues},
    },
};

use super::{
    polygon_filling_range::PolygonFillingRange,
};

pub fn draw_polygon<const N: usize>(
    polygon: &Polygon<N>,
    canvas: &mut DrawingBuffer,
    color: Option<&Color>,
) {
    polygon
        .get_perimeter_lines()
        .into_iter()
        .for_each(|l| draw_line(&l, canvas, color));
}

pub fn fill_polygon<const N: usize>(
    polygon: &Polygon<N>,
    canvas: &mut DrawingBuffer,
    texture: &DynamicImage,
    normal_map: Option<&PlaneBuffer<Vec3>>,
    light_dir: Vec3,
    look_dir: Vec3,
    color: Option<&Color>,
) {
    let mut lines = polygon.get_perimeter_lines();
    lines.iter_mut().for_each(Line::order_by_x);

    let interpolators: Vec<(
        Interpolator<i32>,
        (InterpolationValues, InterpolationValues),
    )> = lines
        .into_iter()
        .map(|l| {
            let b = l.begin;
            let e = l.end;
            let interpolator = Interpolator::from((b.x, e.x));
            let begin_piv = InterpolationValues::from(b);
            let end_piv = InterpolationValues::from(e);
            let interpolation_values = end_piv - begin_piv;
            (interpolator, (interpolation_values, begin_piv))
        })
        .collect();

    let mut x_sorted_points: Vec<_> = polygon.get_points().iter().collect();
    x_sorted_points.sort_unstable_by(|a, b| a.x.cmp(&b.x));
    let polygon_filling_ranges: Vec<_> = x_sorted_points
        .windows(2)
        .map(|two_points| unsafe {
            let (p1, p2) = (two_points.get_unchecked(0), two_points.get_unchecked(1));
            let range = p1.x..p2.x;
            let suitable_interpolators: Vec<&(
                Interpolator<i32>,
                (InterpolationValues, InterpolationValues),
            )> = interpolators
                .iter()
                .filter(|lc| {
                    let lc_range = lc.0.get_interpolation_range();
                    lc_range.start <= range.start && range.end <= lc_range.end
                })
                .collect();
            PolygonFillingRange {
                interpolators: suitable_interpolators,
                range,
            }
        })
        .collect();

    let (width, height) = texture.dimensions();

    for polygon_filling_range in polygon_filling_ranges.into_iter() {
        for x in polygon_filling_range.range {
            let mut yzs: Vec<_> = polygon_filling_range
                .interpolators
                .iter()
                .map(|v| {
                    let mut piv = v.0.interpolate(x, v.1 .0, v.1 .1);
                    piv.color = v.1 .1.color.interpolate(
                        v.1 .0.color,
                        (x - v.0.get_begin()) as i32,
                        (*v.0.get_diff()) as i32,
                    );
                    piv
                })
                .collect();
            yzs.sort_unstable_by(|a, b| a.y.cmp(&b.y));

            for two_ys in yzs.chunks_exact(2) {
                let mut yz1 = *unsafe { two_ys.get_unchecked(0) };
                let mut yz2 = *unsafe { two_ys.get_unchecked(1) };

                if yz1.y > yz2.y {
                    (yz1, yz2) = (yz2, yz1);
                }

                let y1 = yz1.y;
                let y2 = yz2.y;

                let local_interpolator = Interpolator::new(y1, y2);
                let dyz = yz2 - yz1;

                for y in y1..y2 {
                    let interpolated_values = local_interpolator.interpolate(y, dyz, yz1);
                    let InterpolationValues {
                        y,
                        z_depth,
                        uv,
                        mut normal,
                        has_color,
                        ..
                    } = interpolated_values;

                    let uvx = (uv[0] * width as f32) as u32;
                    let uvy = (uv[1] * height as f32) as u32;

                    if let Some(normal_map) = normal_map {
                        // normal = (normal + normal_map[(uvx as usize, uvy as usize)]).normalized();
                        let nm = normal_map[(uvx as usize, uvy as usize)];
                        normal = nm
                        //  * -1.0
                         ;
                    }

                    let visibility = look_dir.dot(normal);
                    if visibility < 0.0 {
                        continue;
                    }

                    let intensity = light_dir.dot(normal).max(0.0);
                    let new_color = if has_color {
                        yz1.color
                            .interpolate(yz2.color, (y - y1) as i32, (y2 - y1) as i32)
                    } else {
                        color
                            .map(|c| *c)
                            .unwrap_or(Color::from(texture.get_pixel(uvx, uvy)))
                    } * intensity;
                    let p = (x as usize, y as usize);
                    let z_val = &mut canvas.get_z_buffer_mut()[p];
                    if *z_val < z_depth {
                        *z_val = z_depth;
                        canvas[p] = new_color;
                    }
                }
            }
        }
    }
}
