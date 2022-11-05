use glam::Mat4;

pub fn create_projection_matrix(distance: f32) -> Mat4 {
    let mut m = Mat4::IDENTITY;
    m.col_mut(2)[3] = -1.0 / distance;
    m
}
