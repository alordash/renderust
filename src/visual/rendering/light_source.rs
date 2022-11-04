use glam::Vec3A;

#[derive(Clone, Copy, Debug)]
pub enum LightSourceKind {
    Linear(Vec3A),
    Ambient,
}

#[derive(Clone, Copy, Debug)]
pub struct LightSource {
    pub kind: LightSourceKind,
    pub spectrum: Vec3A,
    pub concentration: f32
}

impl LightSource {
    pub fn new(kind: LightSourceKind, spectrum: Vec3A, concentration: f32) -> LightSource {
        LightSource { kind, spectrum, concentration }
    }
}
