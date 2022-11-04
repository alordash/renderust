use glam::Vec2;

use crate::{plane_buffer::plane_buffer::PlaneBuffer, visual::drawing_buffer::DrawingBuffer};

const NEIGHBOURS_DIRECTIONS: [Vec2; 8] = [
    Vec2::new(1.0, 0.0),
    Vec2::new(1.0, -1.0),
    Vec2::new(0.0, -1.0),
    Vec2::new(-1.0, -1.0),
    Vec2::new(-1.0, 0.0),
    Vec2::new(-1.0, 1.0),
    Vec2::new(0.0, 1.0),
    Vec2::new(1.0, 1.0),
];

pub fn max_elevation_angle(
    z_buffer: &PlaneBuffer<f32>,
    from: Vec2,
    dir: Vec2,
    z_buffer_depth: f32,
    effect_radius: f32,
) -> f32 {
    let mut max_angle = 0f32;
    let mut t = 0.0;
    let from_z = z_buffer[(from.x as usize, from.y as usize)];
    while t < effect_radius {
        let cur = from + dir * t;
        let pos = (cur.x as usize, cur.y as usize);

        if !z_buffer.contains(pos.0, pos.1) {
            return max_angle;
        }
        let cur_z = z_buffer[pos];
        if cur_z < 0.0 {
            return max_angle;
        }

        let distance = from.distance(cur);
        t += 1.0;
        if distance < 1.0 {
            continue;
        }

        let elevation = (cur_z - from_z) / z_buffer_depth;
        max_angle += elevation / distance.powf(2.0);
    }

    max_angle
}

pub fn render_ambient_occlusion(
    canvas: &mut DrawingBuffer,
    z_buffer_depth: f32,
    effect_radius: f32,
    intensity: f32,
) {
    for x in 0..canvas.get_width() as i32 {
        for y in 0..canvas.get_height() as i32 {
            let z_buffer = canvas.get_z_buffer_mut();
            let pos = (x as usize, y as usize);
            let cur_z = z_buffer[pos];
            if cur_z < 0.0 {
                continue;
            }
            let mut total = 0f32;

            for neighbour_dir in NEIGHBOURS_DIRECTIONS.iter() {
                total += std::f32::consts::FRAC_PI_2
                    - max_elevation_angle(
                        z_buffer,
                        Vec2::new(x as f32, y as f32),
                        *neighbour_dir,
                        z_buffer_depth,
                        effect_radius,
                    ) * intensity;
            }

            total /= std::f32::consts::FRAC_PI_2 * 8.0;
            total = total.powf(200.0);

            canvas[pos] = canvas[pos] * total.clamp(0.0, 1.0);
        }
    }
}
