use crate::{
    math::{
        geometry::{
            primitives::{line::Line, point::Point2D, polygons::triangle::Triangle},
            rect_size::RectSize,
        },
        vector::common_vectors::{vec2f::Vec2f, vec3f::Vec3f},
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
            let v0 = wavefront_obj.vertices[face[0].values()[j]];
            let v1 = wavefront_obj.vertices[face[0].values()[(j + 1) % 3]];
            let x0 = ((v0.x() + 1.0) * w_f32 / 2.0) as isize;
            let y0 = ((v0.y() + 1.0) * h_f32 / 2.0) as isize;
            let x1 = ((v1.x() + 1.0) * w_f32 / 2.0) as isize;
            let y1 = ((v1.y() + 1.0) * h_f32 / 2.0) as isize;

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
    light_dir: Vec3f,
    look_dir: Vec3f,
    color: Option<&Color>,
    use_nm: bool,
) {
    let RectSize { width, height } = canvas.get_size();

    let (w_f32, h_f32) = ((width - 1) as f32, (height - 1) as f32);

    for i in 0..wavefront_obj.faces.len() {
        let face = &wavefront_obj.faces[i];
        let mut world_coords = [Vec3f::default(); 3];
        let mut screen_coords = [Point2D::from(0, 0); 3];

        for j in 0..3_usize {
            let v0 = wavefront_obj.vertices[face[0].values()[j]];
            let x0 = ((v0.x() + 1.0) * w_f32 / 2.0) as isize;
            let y0 = ((v0.y() + 1.0) * h_f32 / 2.0) as isize;
            world_coords[j] = v0;
            let uvidx = face[1].values()[j];
            let uv3d = wavefront_obj.vertex_textures[uvidx];
            screen_coords[j] = Point2D::new_full(
                [x0, y0],
                (1000.0 * world_coords[j].z()) as isize,
                Vec2f::new([uv3d.x(), uv3d.y()]),
                wavefront_obj.vertex_normals[face[0].values()[j]],
            );
        }

        let triangle = Triangle::new(screen_coords);
        fill_triangle(
            &triangle,
            canvas,
            &wavefront_obj.texture,
            if use_nm {
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
