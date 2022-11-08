use glam::Mat4;

use crate::{
    math::geometry::{
        apply_transform_matrix::vertex_apply_transform_matrix, primitives::line::Line,
    },
    visual::{
        color::color::Color,
        drawing_buffer::DrawingBuffer,
        rendering::{
            light_source::{LightSource, LightSourceKind},
            line::line_rasterization::draw_line,
            triangle::triangle_rasterization::render_triangle_mesh,
        },
    },
    wavefront::wavefront_obj::WavefrontObj,
};

use super::{
    wavefront_obj_processing::calculate_wavefront_faces,
    wavefront_render_model::WavefrontRenderModel,
};

pub fn render_wavefront_grid(
    wavefront_obj: &WavefrontObj,
    canvas: &mut DrawingBuffer,
    viewport_matrix: Mat4,
    projection: Mat4,
    view_matrix: Mat4,
    model_matrix: Mat4,
    rotation_matrix: Mat4,
    color: Option<&Color>,
) {
    let transform_matrix =
        viewport_matrix * projection * model_matrix * rotation_matrix * view_matrix;

    for i in 0..wavefront_obj.faces.len() {
        let face = &wavefront_obj.faces[i];
        for j in 0..3_usize {
            let first_vertex = vertex_apply_transform_matrix(
                wavefront_obj.vertices[face[0][j] as usize],
                transform_matrix,
            );
            let second_vertex = vertex_apply_transform_matrix(
                wavefront_obj.vertices[face[0][(j + 1) % 3] as usize],
                transform_matrix,
            );

            if !canvas.contains(first_vertex.x as usize, second_vertex.y as usize)
                || !canvas.contains(first_vertex.x as usize, second_vertex.y as usize)
            {
                continue;
            }

            let line = Line {
                begin: first_vertex,
                end: second_vertex,
            };
            draw_line(&line, canvas, color);
        }
    }
}

pub fn render_wavefront_mesh(
    model: &WavefrontRenderModel,
    canvas: &mut DrawingBuffer,
    mut lights: Vec<LightSource>,
    viewport_matrix: Mat4,
    projection: Mat4,
    view_matrix: Mat4,
    rotation_matrix: Mat4,
) {
    let wavefront_obj = &model.obj;

    for light in lights.iter_mut() {
        match &mut light.kind {
            LightSourceKind::Linear { dir, .. } => {
                *dir = vertex_apply_transform_matrix(*dir, rotation_matrix.transpose().inverse())
                    .normalize();
            }
            _ => (),
        }
    }

    let faces = calculate_wavefront_faces(
        model,
        viewport_matrix,
        projection,
        view_matrix,
        rotation_matrix,
        0.0..canvas.get_width() as f32,
        0.0..canvas.get_height() as f32,
    );

    for face in faces.iter() {
        render_triangle_mesh(
            &face,
            canvas,
            &wavefront_obj.texture,
            &mut lights,
            if model.use_normal_map {
                wavefront_obj.normal_map.as_ref()
            } else {
                None
            },
            if model.use_spec_map {
                wavefront_obj.spec_map.as_ref()
            } else {
                None
            },
            if model.use_glow_map {
                wavefront_obj.glow_map.as_ref()
            } else {
                None
            },
        );
    }
}
