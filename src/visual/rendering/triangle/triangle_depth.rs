use crate::{
    math::interpolation::Interpolator,
    plane_buffer::plane_buffer::PlaneBuffer,
    visual::{rendering::triangle::interpolation_values::InterpolationValues, vertex::Vertex},
};

pub fn render_triangle_depth(vertices: &[Vertex; 3], depth_buffer: &mut PlaneBuffer<f32>) {
    let mut vertices_sorted_by_x = vertices.clone();
    vertices_sorted_by_x.sort_unstable_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

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
                if !depth_buffer.contains(p.0, p.1) {
                    continue;
                }

                let local_v = local_calc.interpolate(y, local_d_v, v1);

                let InterpolationValues {
                    z_depth,
                    ..
                } = local_v;

                let z_val = &mut depth_buffer[p];
                if *z_val > z_depth {
                    continue;
                }

                *z_val = z_depth;
            }
        }
    };

    filler(l_calc, l_v, m_v);
    filler(r_calc, m_v, r_v);
}
