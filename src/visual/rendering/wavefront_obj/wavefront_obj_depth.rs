use glam::Mat4;

use crate::{
    plane_buffer::plane_buffer::PlaneBuffer,
    visual::rendering::triangle::triangle_depth::render_triangle_depth,
};

use super::{
    wavefront_obj_processing::calculate_wavefront_faces,
    wavefront_render_model::WavefrontRenderModel,
};

pub fn render_wavefront_depth(
    model: &WavefrontRenderModel,
    depth_buffer: &mut PlaneBuffer<f32>,
    viewport_matrix: Mat4,
    projection: Mat4,
    view_matrix: Mat4,
    rotation_matrix: Mat4,
) {
    let faces = calculate_wavefront_faces(
        model,
        viewport_matrix,
        projection,
        view_matrix,
        rotation_matrix,
        0.0..depth_buffer.get_width() as f32,
        0.0..depth_buffer.get_height() as f32,
    );

    for face in faces.iter() {
        render_triangle_depth(&face, depth_buffer);
    }
}
