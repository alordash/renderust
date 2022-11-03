use glam::{Mat4, Vec3A};

#[derive(Clone, Copy, Debug)]
pub struct RenderConfig {
    pub look: LookConfig,
    pub camera: CameraConfig,
    pub light_dir: Vec3A,
    pub spin_light: bool,
    pub transform_matrixes: TransformMatrixes,
}

#[derive(Clone, Copy, Debug)]
pub struct LookConfig {
    pub from: Vec3A,
    pub to: Vec3A,
    pub up: Vec3A,
}

#[derive(Clone, Copy, Debug)]
pub struct CameraConfig {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct TransformMatrixes {
    pub view_matrix: Mat4,
    pub projection: Mat4,
    pub viewport_matrix: Mat4,
}
