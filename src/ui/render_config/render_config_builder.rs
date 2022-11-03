use glam::{Mat4, Vec3A};

use crate::visual::rendering::view_matrix::create_view_matrix;

use super::render_config::{CameraConfig, LookConfig, RenderConfig, TransformMatrixes};

pub struct RenderConfigBuilder {
    look: Option<LookConfig>,
    camera: Option<CameraConfig>,
    light_dir: Option<Vec3A>,
    spin_light: Option<bool>,
    transform_matrixes: Option<TransformMatrixes>,
}

impl RenderConfigBuilder {
    pub fn new() -> RenderConfigBuilder {
        RenderConfigBuilder {
            look: None,
            camera: None,
            light_dir: None,
            spin_light: None,
            transform_matrixes: None,
        }
    }

    pub fn look(mut self, look: LookConfig) -> RenderConfigBuilder {
        self.look = Some(look);
        self
    }

    pub fn camera(mut self, camera: CameraConfig) -> RenderConfigBuilder {
        self.camera = Some(camera);
        self
    }

    pub fn light_dir(mut self, light_dir: Vec3A) -> RenderConfigBuilder {
        self.light_dir = Some(light_dir);
        self
    }

    pub fn spin_light(mut self, spin_light: bool) -> RenderConfigBuilder {
        self.spin_light = Some(spin_light);
        self
    }

    pub fn transform_matrixes(mut self, viewport_matrix: Mat4) -> RenderConfigBuilder {
        let LookConfig { from, to, up } = self.look.unwrap();
        let mut transform_matrixes = TransformMatrixes {
            view_matrix: create_view_matrix(from, to, up),
            projection: Mat4::IDENTITY,
            viewport_matrix,
        };
        transform_matrixes.projection.col_mut(2)[3] = -1.0 / from.distance(to);
        self.transform_matrixes = Some(transform_matrixes);
        self
    }

    pub fn build(self) -> RenderConfig {
        RenderConfig {
            look: self.look.unwrap_or(LookConfig {
                from: Vec3A::Z * 5.0,
                to: Vec3A::ZERO,
                up: Vec3A::Y,
            }),
            camera: self.camera.unwrap_or(CameraConfig {
                yaw: 0.0,
                pitch: 0.0,
                distance: 5.0,
            }),
            light_dir: self
                .light_dir
                .unwrap_or(Vec3A::new(0.0, 1.0, 1.0).normalize()),
            spin_light: self.spin_light.unwrap_or(false),
            transform_matrixes: self.transform_matrixes.unwrap(),
        }
    }
}
