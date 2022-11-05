pub mod line {
    pub mod line_iterators;
    pub mod line_rasterization;
}
pub mod polygon {
    pub mod polygon_filling_range;
}
pub mod wavefront_obj {
    pub mod wavefront_obj_rendering;
    pub mod wavefront_render_model;
}
pub mod triangle {
    pub mod triangle_rasterization;
}
pub mod interpolation_values;
pub mod matrix {
    pub mod view_matrix;
    pub mod viewport_matrix;
    pub mod projection_matrix;
}
pub mod ambient_occlusion;
pub mod light_source;
