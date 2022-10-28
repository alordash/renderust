pub fn spherical_to_cartesian_yzx(theta: f32, phi: f32, r: f32) -> (f32, f32, f32) {
    (
        r * theta.sin() * phi.sin(),
        r * theta.cos(),
        r * theta.sin() * phi.cos(),
    )
}
