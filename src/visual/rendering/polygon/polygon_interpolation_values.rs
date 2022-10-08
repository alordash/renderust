use crate::{
    derive_div_by, derive_mul_by, derive_self_add, derive_self_sub,
    math::{
        geometry::primitives::point::Point2D,
        vector::common_vectors::{vec2f::Vec2f, vec3f::Vec3f},
    },
    visual::color::color::Color,
};

#[derive(Clone, Copy)]
pub struct PolygonInterpolationValues {
    pub y: isize,
    pub z_depth: isize,
    pub uv: Vec2f,
    pub normal: Vec3f,
    pub color: Color,
    pub has_color: bool,
}

impl From<Point2D> for PolygonInterpolationValues {
    fn from(p: Point2D) -> Self {
        PolygonInterpolationValues {
            y: p.y(),
            z_depth: *p.get_z_depth(),
            uv: *p.get_uv(),
            normal: *p.get_normal(),
            color: p.get_color().unwrap_or(Color::from_rgb(0, 0, 0)),
            has_color: p.get_color().is_some(),
        }
    }
}

derive_self_add!(PolygonInterpolationValues, y, z_depth, uv, normal);
derive_self_sub!(PolygonInterpolationValues, y, z_depth, uv, normal);
derive_mul_by!(PolygonInterpolationValues, isize, y, z_depth, uv, normal);
derive_div_by!(PolygonInterpolationValues, isize, y, z_depth, uv, normal);
