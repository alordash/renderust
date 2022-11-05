use glam::{Mat4, Vec3A};

use crate::visual::rendering::{light_source::LightSource, wavefront_obj::wavefront_render_model::WavefrontRenderModel, matrix::view_matrix::create_view_matrix};

#[derive(Clone, Debug, Builder)]
pub struct RenderConfig {
    pub look: LookConfig,
    pub camera: CameraConfig,
    pub lights: Vec<LightSource>,
    pub ambient_occlusion: AmbientOcclusionConfig,
    #[builder(setter(custom))]
    pub transform_matrixes: TransformMatrixes,
    pub models: Vec<WavefrontRenderModel>
}

#[derive(Clone, Copy, Debug)]
pub struct LookConfig {
    pub from: Vec3A,
    pub to: Vec3A,
    pub up: Vec3A,
}

#[derive(Clone, Copy, Debug)]
pub struct CameraConfig {
    pub pitch: f32,
    pub yaw: f32,
    pub distance: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct AmbientOcclusionConfig {
    pub apply: bool,
    pub effect_radius: f32,
    pub intensity: f32
}

#[derive(Clone, Copy, Debug)]
pub struct TransformMatrixes {
    pub view_matrix: Mat4,
    pub projection: Mat4,
    pub viewport_matrix: Mat4,
}

impl RenderConfigBuilder {
    pub fn transform_matrixes(&mut self, viewport_matrix: Mat4) -> &mut Self {
        let mut new = self;
        let LookConfig { from, to, up } = new.look.unwrap();
        let mut transform_matrixes = TransformMatrixes {
            view_matrix: create_view_matrix(from, to, up),
            projection: Mat4::IDENTITY,
            viewport_matrix,
        };
        transform_matrixes.projection.col_mut(2)[3] = -1.0 / from.distance(to);
        new.transform_matrixes = Some(transform_matrixes);
        new
    }
}
