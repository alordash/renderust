pub struct WaveFrontObjSource {
    pub model_path: &'static str,
    pub texture_path: &'static str,
    pub normal_map_path: Option<&'static str>,
    pub spec_map_path: Option<&'static str>,
    pub glow_map_path: Option<&'static str>
}

impl WaveFrontObjSource {
    pub const fn new(
        model_path: &'static str,
        texture_path: &'static str,
        normal_map_path: Option<&'static str>,
        spec_map_path: Option<&'static str>,
        glow_map_path: Option<&'static str>
    ) -> WaveFrontObjSource {
        WaveFrontObjSource {
            model_path,
            texture_path,
            normal_map_path,
            spec_map_path,
            glow_map_path
        }
    }
}
