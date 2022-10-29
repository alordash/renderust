pub struct WaveFrontObjSource {
    pub model_path: &'static str,
    pub texture_path: &'static str,
    pub normal_map_path: &'static str,
}

impl WaveFrontObjSource {
    pub const fn new(
        model_path: &'static str,
        texture_path: &'static str,
        normal_map_path: &'static str,
    ) -> WaveFrontObjSource {
        WaveFrontObjSource {
            model_path,
            texture_path,
            normal_map_path,
        }
    }
}
