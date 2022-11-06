use glam::{Vec3A, Mat4};

use crate::plane_buffer::plane_buffer::PlaneBuffer;

#[derive(Clone, Debug)]
pub enum LightSourceKind {
    Linear {
        dir: Vec3A,
        shadow_buffer: Option<PlaneBuffer<f32>>,
        transform_matrix: Option<Mat4>
    },
    Ambient,
}

#[derive(Clone, Debug)]
pub struct LightSource {
    pub kind: LightSourceKind,
    pub spectrum: Vec3A,
    pub concentration: f32,
}

impl LightSource {
    pub fn new(kind: LightSourceKind, spectrum: Vec3A, concentration: f32) -> LightSource {
        LightSource {
            kind,
            spectrum,
            concentration,
        }
    }
}
