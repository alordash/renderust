use glam::{Mat3, Mat4, Vec2, Vec3, Vec4, Vec4Swizzles};

use crate::{
    math::geometry::{
        primitives::{line::Line, point::Point2D, polygons::triangle::Triangle},
        rect_size::RectSize,
    },
    visual::{
        color::color::Color,
        drawing_buffer::DrawingBuffer,
        rendering::{
            line::line_rasterization::draw_line, polygon::polygon_rasterization::fill_polygon,
            triangle::triangle_rasterization::fill_triangle,
        },
    },
    wavefront::wavefront_obj::WavefrontObj,
};

pub fn render_wavefront_grid(
    wavefront_obj: &WavefrontObj,
    canvas: &mut DrawingBuffer,
    color: Option<&Color>,
) {
    let RectSize { width, height } = canvas.get_size();

    let (w_f32, h_f32) = ((width - 1) as f32, (height - 1) as f32);

    for i in 0..wavefront_obj.faces.len() {
        let face = &wavefront_obj.faces[i];
        for j in 0..3_usize {
            let v0 = wavefront_obj.vertices[face[0][j] as usize];
            let v1 = wavefront_obj.vertices[face[0][(j + 1) % 3] as usize];
            let x0 = ((v0.x + 1.0) * w_f32 / 2.0) as i32;
            let y0 = ((v0.y + 1.0) * h_f32 / 2.0) as i32;
            let x1 = ((v1.x + 1.0) * w_f32 / 2.0) as i32;
            let y1 = ((v1.y + 1.0) * h_f32 / 2.0) as i32;

            let begin = Point2D::from(x0, y0);
            let end = Point2D::from(x1, y1);
            let line = Line { begin, end };
            draw_line(&line, canvas, color);
        }
    }
}

pub fn render_wavefront_mesh(
    wavefront_obj: &WavefrontObj,
    canvas: &mut DrawingBuffer,
    light_dir: Vec3,
    look_dir: Vec3,
    color: Option<&Color>,
    use_normal_map: bool,
    projection: Mat4,
    model_view_matrix: Mat4,
) {
    let RectSize { width, height } = canvas.get_size();

    let (w_f32, h_f32) = ((width - 1) as f32, (height - 1) as f32);

    for i in 0..wavefront_obj.faces.len() {
        let face = &wavefront_obj.faces[i];
        let mut screen_coords = [Point2D::from(0, 0); 3];

        for j in 0..3_usize {
            let vertex4 =
                projection * Vec4::from((wavefront_obj.vertices[face[0][j] as usize], 1.0));
            let vertex = Vec3::from(vertex4.xyz()) / vertex4.w;
            // let mut vertex4 = Vec4::from((vertex, 1.0));
            // let mut transform_matrix = Mat4::from_diagonal(Vec4::new(1.0, 1.0, 1.0, 1.0));
            // transform_matrix.col_mut(2)[3] = -1.0 / camera_z_pos;
            // vertex4 = transform_matrix * vertex4;
            // vertex = vertex4.xyz() / vertex4[3];

            let x = ((vertex.x + 1.0) * w_f32 / 2.0) as i32;
            let y = ((vertex.y + 1.0) * h_f32 / 2.0) as i32;
            let uvidx = face[1][j] as usize;
            let uv3d = wavefront_obj.vertex_textures[uvidx];
            screen_coords[j] = Point2D::new_full(
                [x, y],
                (1000.0 * vertex.z) as i32,
                Vec2::new(uv3d.x, uv3d.y),
                wavefront_obj.vertex_normals[face[0][j] as usize],
            );
        }

        let triangle = Triangle::new(screen_coords);
        fill_triangle(
            &triangle,
            canvas,
            &wavefront_obj.texture,
            if use_normal_map {
                Some(&wavefront_obj.normal_map)
            } else {
                None
            },
            light_dir,
            look_dir,
            color,
        );
        // fill_polygon(
        //     &triangle,
        //     canvas,
        //     &wavefront_obj.texture,
        //     if use_nm {
        //         Some(&wavefront_obj.normal_map)
        //     } else {
        //         None
        //     },
        //     light_dir,
        //     look_dir,
        //     color,
        // );
    }
}
