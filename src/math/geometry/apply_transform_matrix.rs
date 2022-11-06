use glam::{Mat4, Vec3A, Vec4, Vec4Swizzles};

pub fn vertex_apply_transform_matrix(vertex: Vec3A, matrix: Mat4) -> Vec3A {
    let vertex4 = matrix * Vec4::from((vertex, 1.0));
    Vec3A::from(vertex4.xyz()) / vertex4.w
}

pub fn vector_apply_transform_matrix(vector: Vec3A, matrix: Mat4) -> Vec3A {
    (matrix * Vec4::from((vector, 0.0))).xyz().into()
}
