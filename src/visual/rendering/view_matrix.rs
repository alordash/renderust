use glam::{Mat4, Vec3};

pub fn create_view_matrix_and_look_dir(from: Vec3, to: Vec3, up: Vec3) -> (Mat4, Vec3) {
    let z = (from - to).normalize();
    let x = up.cross(z).normalize();
    let y = z.cross(x).normalize();

    let mut m_inv = Mat4::IDENTITY;
    let mut tr = Mat4::IDENTITY;
    for i in 0..3 {
        m_inv.col_mut(0)[i] = x[i];
        m_inv.col_mut(1)[i] = y[i];
        m_inv.col_mut(2)[i] = z[i];
        tr.col_mut(i)[3] = -to[i];
    }

    (m_inv * tr, z)
}