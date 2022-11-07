use glam::Mat4;

pub fn create_rotation_matrix(yaw: f32, pitch: f32) -> Mat4 {
    Mat4::from_rotation_y(yaw) * Mat4::from_rotation_x(pitch)
}