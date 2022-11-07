use std::ops::Range;

use glam::{Mat4, Vec2};

use crate::{
    math::geometry::apply_transform_matrix::{
        vector_apply_transform_matrix, vertex_apply_transform_matrix,
    },
    visual::vertex::Vertex,
};

use super::wavefront_render_model::WavefrontRenderModel;

pub fn calculate_wavefront_faces(
    model: &WavefrontRenderModel,
    viewport_matrix: Mat4,
    projection: Mat4,
    view_matrix: Mat4,
    rotation_matrix: Mat4,
    x_range: Range<f32>,
    y_range: Range<f32>,
) -> Vec<[Vertex; 3]> {
    let wavefront_obj = &model.obj;
    let transform_matrix = viewport_matrix * projection * model.model_matrix * rotation_matrix * view_matrix;
    let inverse_transposed_transform_matrix = rotation_matrix.transpose().inverse();

    let faces = wavefront_obj
        .faces
        .iter()
        .map(|face| {
            let mut screen_vertices = [Vertex::default(); 3];

            let mut skip = false;
            for j in 0..3_usize {
                let vertex = vertex_apply_transform_matrix(
                    wavefront_obj.vertices[face[0][j] as usize],
                    transform_matrix,
                );

                let normal = vector_apply_transform_matrix(
                    wavefront_obj.vertex_normals[face[0][j] as usize],
                    inverse_transposed_transform_matrix,
                )
                .normalize();

                let uvidx = face[1][j] as usize;
                let uv3d = wavefront_obj.vertex_textures[uvidx];
                screen_vertices[j] = Vertex::new(vertex, Vec2::new(uv3d.x, uv3d.y), normal);
                let pos = screen_vertices[j].screen_pos;

                if !x_range.contains(&pos.x) || !y_range.contains(&pos.y) {
                    skip = true;
                    break;
                }
            }
            if skip {
                None
            } else {
                Some(screen_vertices)
            }
        })
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect();
    faces
}
