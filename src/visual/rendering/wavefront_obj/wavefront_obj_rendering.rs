use glam::{Mat4, Vec2, Vec3A, Vec4, Vec4Swizzles};

use crate::{
    math::geometry::primitives::line::Line,
    visual::{
        color::color::Color,
        drawing_buffer::DrawingBuffer,
        rendering::{
            light_source::LightSource, line::line_rasterization::draw_line,
            triangle::triangle_rasterization::fill_triangle,
        },
        vertex::Vertex,
    },
    wavefront::wavefront_obj::WavefrontObj,
};

use super::wavefront_render_model::WavefrontRenderModel;

pub fn render_wavefront_grid(
    wavefront_obj: &WavefrontObj,
    canvas: &mut DrawingBuffer,
    viewport_matrix: Mat4,
    projection: Mat4,
    view_matrix: Mat4,
    model_matrix: Mat4,
    color: Option<&Color>,
) {
    let transform_matrix = viewport_matrix * projection * view_matrix * model_matrix;

    for i in 0..wavefront_obj.faces.len() {
        let face = &wavefront_obj.faces[i];
        for j in 0..3_usize {
            let first_source_vertex = wavefront_obj.vertices[face[0][j] as usize];
            let first_vertex4 = transform_matrix * Vec4::from((first_source_vertex, 1.0));
            let first_vertex = Vec3A::from(first_vertex4.xyz()) / first_vertex4.w;

            let second_source_vertex = wavefront_obj.vertices[face[0][(j + 1) % 3] as usize];
            let second_vertex4 = transform_matrix * Vec4::from((second_source_vertex, 1.0));
            let second_vertex = Vec3A::from(second_vertex4.xyz()) / second_vertex4.w;

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
    lights: Vec<LightSource>,
    viewport_matrix: Mat4,
    projection: Mat4,
    view_matrix: Mat4,
) {
    let wavefront_obj = &model.obj;
    let transform_matrix = viewport_matrix * projection * model.model_matrix * view_matrix;
    let inverse_transposed_transform_matrix = transform_matrix.transpose().inverse();

    for i in 0..wavefront_obj.faces.len() {
        let face = &wavefront_obj.faces[i];
        let mut screen_vertices = [Vertex::default(); 3];

        let mut skip = false;
        for j in 0..3_usize {
            let source_vertex = wavefront_obj.vertices[face[0][j] as usize];
            let vertex4 = transform_matrix * Vec4::from((source_vertex, 1.0));
            let vertex = Vec3A::from(vertex4.xyz()) / vertex4.w;

            let source_normal = wavefront_obj.vertex_normals[face[0][j] as usize];
            let normal = Vec3A::from(
                (inverse_transposed_transform_matrix * Vec4::from((source_normal, 0.0))).xyz(),
            )
            .normalize();

            let uvidx = face[1][j] as usize;
            let uv3d = wavefront_obj.vertex_textures[uvidx];
            screen_vertices[j] = Vertex::new(vertex, Vec2::new(uv3d.x, uv3d.y), normal);

            if !canvas.get_z_buffer().contains(
                screen_vertices[j].screen_pos.x as usize,
                screen_vertices[j].screen_pos.y as usize,
            ) {
                skip = true;
                break;
            }
        }
        if skip {
            continue;
        }

        fill_triangle(
            &screen_vertices,
            canvas,
            &wavefront_obj.texture,
            &lights,
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
