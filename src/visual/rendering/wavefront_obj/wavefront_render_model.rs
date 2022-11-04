use glam::Mat4;

use crate::wavefront::wavefront_obj::WavefrontObj;

#[derive(Clone, Debug, Builder)]
pub struct WavefrontRenderModel {
    pub obj: WavefrontObj,
    pub use_normal_map: bool,
    pub model_matrix: Mat4,
}

impl WavefrontRenderModel {
    pub fn from_wavefront_obj(wavefront_obj: WavefrontObj) -> WavefrontRenderModel {
        WavefrontRenderModelBuilder::default()
            .obj(wavefront_obj)
            .use_normal_map(false)
            .model_matrix(Mat4::IDENTITY)
            .build()
            .unwrap()
    }
}
