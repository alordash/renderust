use glam::{Mat4, Vec3A};

pub fn create_view_matrix(from: Vec3A, to: Vec3A, up: Vec3A) -> Mat4 {
    let z = (from - to).normalize_or_zero();
    let x = up.cross(z).normalize_or_zero();
    let y = z.cross(x).normalize_or_zero();

    let mut m_inv = Mat4::IDENTITY;
    let mut tr = Mat4::IDENTITY;
    for i in 0..3 {
        m_inv.col_mut(0)[i] = x[i];
        m_inv.col_mut(1)[i] = y[i];
        m_inv.col_mut(2)[i] = z[i];
        tr.col_mut(i)[3] = -to[i];
    }

    m_inv * tr
}
