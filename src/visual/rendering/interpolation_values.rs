use glam::{Vec2, Vec3};

use crate::{
    derive_div_by, derive_mul_by, derive_self_add, derive_self_sub,
    math::{
        geometry::primitives::point::Point2D,
    },
    visual::color::color::Color,
};

#[derive(Clone, Copy)]
pub struct InterpolationValues {
    pub y: i32,
    pub z_depth: i32,
    pub uv: Vec2,
    pub normal: Vec3,
    pub color: Color,
    pub has_color: bool,
}

impl From<Point2D> for InterpolationValues {
    fn from(p: Point2D) -> Self {
        InterpolationValues {
            y: p.y,
            z_depth: *p.get_z_depth(),
            uv: *p.get_uv(),
            normal: *p.get_normal(),
            color: p.get_color().unwrap_or(Color::from_rgb(0, 0, 0)),
            has_color: p.get_color().is_some(),
        }
    }
}

derive_self_add!(InterpolationValues, y, z_depth, uv, normal);
derive_self_sub!(InterpolationValues, y, z_depth, uv, normal);
derive_mul_by!(InterpolationValues, i32, y, i32, z_depth, i32, uv, f32, normal, f32);
derive_div_by!(InterpolationValues, i32, y, i32, z_depth, i32, uv, f32, normal, f32);