pub fn spherical_to_cartesian_yzx(yaw: f32, pitch: f32, r: f32) -> (f32, f32, f32) {
    (
        r * pitch.sin() * yaw.sin(),
        r * pitch.cos(),
        r * pitch.sin() * yaw.cos(),
    )
}
