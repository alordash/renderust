use glam::{Vec2, Vec3A};

use crate::{
    derive_div_by, derive_mul_by, derive_self_add, derive_self_sub, visual::vertex::Vertex,
};

#[derive(Clone, Copy)]
pub struct InterpolationValues {
    pub y: f32,
    pub z_depth: f32,
    pub real_pos: Vec3A,
    pub uv: Vec2,
    pub normal: Vec3A,
}

impl From<Vertex> for InterpolationValues {
    fn from(v: Vertex) -> Self {
        InterpolationValues {
            y: v.y,
            z_depth: v.z,
            real_pos: v.real_pos,
            uv: v.uv,
            normal: v.normal,
        }
    }
}

derive_self_add!(InterpolationValues, y, z_depth, real_pos, uv, normal);
derive_self_sub!(InterpolationValues, y, z_depth, real_pos, uv, normal);
derive_mul_by!(
    InterpolationValues,
    i32,
    y,
    f32,
    z_depth,
    f32,
    real_pos,
    f32,
    uv,
    f32,
    normal,
    f32
);
derive_div_by!(
    InterpolationValues,
    i32,
    y,
    f32,
    z_depth,
    f32,
    real_pos,
    f32,
    uv,
    f32,
    normal,
    f32
);
