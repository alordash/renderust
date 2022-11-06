use glam::Mat4;

use crate::wavefront::wavefront_obj::WavefrontObj;

#[derive(Clone, Debug, Builder)]
pub struct WavefrontRenderModel {
    pub obj: WavefrontObj,
    pub use_normal_map: bool,
    pub use_spec_map: bool,
    pub use_glow_map: bool,
    pub use_self_shadowing: bool,
    pub model_matrix: Mat4,
}

impl From<WavefrontObj> for WavefrontRenderModel {
    fn from(wavefront_obj: WavefrontObj) -> Self {
        WavefrontRenderModelBuilder::default()
            .obj(wavefront_obj)
            .use_normal_map(false)
            .use_spec_map(false)
            .use_glow_map(false)
            .use_self_shadowing(false)
            .model_matrix(Mat4::IDENTITY)
            .build()
            .unwrap()
    }
}
