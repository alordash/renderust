use glam::{Mat4, Vec2, Vec3A, Vec4, Vec4Swizzles};

use crate::{
    math::geometry::{
        primitives::{line::Line, point::Point2D, polygons::triangle::Triangle},
        rect_size::RectSize,
    },
    visual::{
        color::color::Color,
        drawing_buffer::DrawingBuffer,
        rendering::{
            line::line_rasterization::draw_line, triangle::triangle_rasterization::fill_triangle,
        },
    },
    wavefront::wavefront_obj::WavefrontObj,
};

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
            let first_vertex = first_vertex4.xyz() / first_vertex4.w;

            let second_source_vertex = wavefront_obj.vertices[face[0][(j + 1) % 3] as usize];
            let second_vertex4 = transform_matrix * Vec4::from((second_source_vertex, 1.0));
            let second_vertex = second_vertex4.xyz() / second_vertex4.w;

            let begin = Point2D::from(first_vertex.x as i32, first_vertex.y as i32);
            let end = Point2D::from(second_vertex.x as i32, second_vertex.y as i32);

            if !canvas.contains(begin.x as usize, begin.y as usize)
                || !canvas.contains(end.x as usize, end.y as usize)
            {
                continue;
            }

            let line = Line { begin, end };
            draw_line(&line, canvas, color);
        }
    }
}

pub fn render_wavefront_mesh(
    wavefront_obj: &WavefrontObj,
    canvas: &mut DrawingBuffer,
    light_dir: Vec3A,
    viewport_matrix: Mat4,
    projection: Mat4,
    view_matrix: Mat4,
    model_matrix: Mat4,
    use_normal_map: bool,
    z_buffer_depth: f32,
) {
    let transform_matrix = viewport_matrix * projection * view_matrix * model_matrix;
    let inverse_transposed_transform_matrix = transform_matrix.transpose().inverse();

    for i in 0..wavefront_obj.faces.len() {
        let face = &wavefront_obj.faces[i];
        let mut screen_coords = [Point2D::from(0, 0); 3];

        let mut skip = false;
        for j in 0..3_usize {
            let source_vertex = wavefront_obj.vertices[face[0][j] as usize];
            let vertex4 = transform_matrix * Vec4::from((source_vertex, 1.0));
            let vertex = vertex4.xyz() / vertex4.w;

            let source_normal = wavefront_obj.vertex_normals[face[0][j] as usize];
            let normal = (inverse_transposed_transform_matrix * Vec4::from((source_normal, 0.0)))
                .xyz()
                .normalize();

            let uvidx = face[1][j] as usize;
            let uv3d = wavefront_obj.vertex_textures[uvidx];
            screen_coords[j] = Point2D::new_full(
                [vertex.x as i32, vertex.y as i32],
                vertex.z,
                Vec2::new(uv3d.x, uv3d.y),
                normal.into(),
            );
            if !canvas.get_z_buffer().contains(
                screen_coords[j].coords.x as usize,
                screen_coords[j].coords.y as usize,
            ) {
                skip = true;
                break;
            }
        }
        if skip {
            continue;
        }

        let triangle = Triangle::new(screen_coords);
        fill_triangle(
            &triangle,
            canvas,
            &wavefront_obj.texture,
            &wavefront_obj.normal_map,
            light_dir,
            use_normal_map,
            z_buffer_depth,
        );
    }
}
