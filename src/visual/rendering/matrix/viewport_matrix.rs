use glam::Mat4;

pub fn create_view_port_matrix(x: f32, y: f32, width: f32, height: f32, depth: f32) -> Mat4 {
    let mut m = Mat4::IDENTITY;
    let (half_width, half_height, half_depth) = (width / 2.0, height / 2.0, depth / 2.0);
    m.col_mut(3)[0] = x + half_width;
    m.col_mut(3)[1] = y + half_height;
    m.col_mut(3)[2] = half_depth;

    m.col_mut(0)[0] = half_width;
    m.col_mut(1)[1] = half_height;
    m.col_mut(2)[2] = half_depth;

    m
}
