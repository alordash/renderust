#![feature(min_specialization)]

use glam::{Mat4, Vec3};
use math::{
    geometry::{
        primitives::{point::Point2D, polygon::Polygon},
        rect_size::RectSize,
    },
    spherical_coordinate_system::spherical_to_cartesian_yzx,
};

pub mod math;
pub mod parsing;
pub mod plane_buffer;
pub mod visual;
pub mod wavefront;

use minifb::{Key, ScaleMode, Window, WindowOptions};
use plane_buffer::plane_buffer::PlaneBufferCreateOption;
use std::{fs::File, time::Instant};
use visual::{
    color::color::Color,
    drawing_buffer::DrawingBuffer,
    rendering::{
        polygon::polygon_rasterization::fill_polygon,
        triangle::triangle_rasterization::fill_triangle,
        view_matrix::create_view_matrix,
        viewport_matrix::create_view_port_matrix,
        wavefront_obj::wavefront_obj_rendering::{render_wavefront_grid, render_wavefront_mesh},
    },
};
use wavefront::{wavefront_obj::WavefrontObj, wavefront_obj_sources::WaveFrontObjSource};

const BUFFER_WIDTH: usize = 1000;
const BUFFER_HEIGHT: usize = 1000;

const WINDOW_WIDTH: usize = 1000;
const WINDOW_HEIGHT: usize = 1000;

const AFRO_MODEL: WaveFrontObjSource = WaveFrontObjSource::new(
    "./resources/african_head.obj",
    "./resources/african_head_diffuse.tga",
    "./resources/african_head_nm_tangent.tga",
);

const FLOOR_MODEL: WaveFrontObjSource = WaveFrontObjSource::new(
    "./resources/floor.obj",
    "./resources/floor_diffuse.tga",
    "./resources/floor_nm_tangent.tga",
);

const DIABLO_MODEL: WaveFrontObjSource = WaveFrontObjSource::new(
    "./resources/diablo3_pose.obj",
    "./resources/diablo3_pose_diffuse.tga",
    "./resources/diablo3_pose_nm_tangent.tga",
);

const POLYGON_SIZE: usize = 3;

fn main() -> Result<(), String> {
    // Allocate the output buffer.
    let mut draw_buffer =
        DrawingBuffer::new(BUFFER_WIDTH, BUFFER_HEIGHT, PlaneBufferCreateOption::Blank);

    let mut window = Window::new(
        "Press ESC to exit",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::Center,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open Window");
    let mut is_mouse_pressed = false;

    let mut width_scale = WINDOW_WIDTH as f32 / BUFFER_WIDTH as f32;
    let mut height_scale = WINDOW_HEIGHT as f32 / BUFFER_HEIGHT as f32;

    let mut t: f32 = 0.0;
    let time_step = 0.05;

    let mut light_dir = Vec3::new(0.0, 0.0, 1.0).normalize();
    let look_dir = Vec3::new(0.0, 0.0, 1.0).normalize();

    let mut polygon_points_z_depth = 5000i32;

    let afro_obj = WavefrontObj::from_sources_struct(&AFRO_MODEL)?;
    let floor_obj = WavefrontObj::from_sources_struct(&FLOOR_MODEL)?;
    let diablo_obj = WavefrontObj::from_sources_struct(&DIABLO_MODEL)?;

    let mut points: Vec<Point2D> = Vec::new();

    let mut from = Vec3::new(0.0, 0.0, 10.0);
    let to = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::Y;

    let mut spin_light = true;
    let mut light_spin_t = 0.0;

    let mut cam_angle_theta = 0.5;
    let mut cam_angle_phi = 0.0;
    let mut cam_distance = 5.0;

    let mut view_matrix = create_view_matrix(from, to, up);
    let (w_f32, h_f32) = (
        draw_buffer.get_width() as f32,
        draw_buffer.get_height() as f32,
    );
    let viewport_matrix = create_view_port_matrix(
        w_f32 * 0.125,
        h_f32 * 0.125,
        w_f32 / 1.25,
        h_f32 / 1.25,
        255.0,
    );

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let start = Instant::now();

        let new_size: RectSize = window.get_size().into();
        if draw_buffer.get_size() != new_size {
            width_scale = new_size.width as f32 / BUFFER_WIDTH as f32;
            height_scale = new_size.height as f32 / BUFFER_HEIGHT as f32;
        }

        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            spin_light = !spin_light;
        }

        let light_angle: f32 = light_spin_t;
        light_dir = Vec3::new(light_angle.sin(), 0.0, light_angle.cos()).normalize();

        if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
            let window_size = RectSize::from(window.get_size());

            cam_angle_theta = ((y / window_size.height as f32) * std::f32::consts::PI).max(0.00001);
            cam_angle_phi = ((x - window_size.width as f32 / 2.0) / window_size.width as f32)
                * std::f32::consts::PI
                * 2.0;

            from = spherical_to_cartesian_yzx(cam_angle_theta, cam_angle_phi, cam_distance).into();
            
            view_matrix = create_view_matrix(from, to, up);
        }
        draw_buffer.get_z_buffer_mut().clean_with(&i32::MIN);
        draw_buffer.clean();

        let mut projection = Mat4::IDENTITY;
        projection.col_mut(2)[3] = -1.0 / from.distance(to);

        render_wavefront_mesh(
            &diablo_obj,
            &mut draw_buffer,
            light_dir,
            look_dir,
            None,
            window.is_key_down(Key::LeftShift),
            projection,
            view_matrix,
            viewport_matrix,
        );

        if window.get_mouse_down(minifb::MouseButton::Left) {
            if !is_mouse_pressed {
                is_mouse_pressed = true;
                if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
                    let y = new_size.height as f32 - y - 1.0;
                    let mut point =
                        Point2D::from((x / width_scale) as i32, (y / height_scale) as i32);
                    *point.get_normal_mut() = Vec3::new(1.0, 0.0, 0.0);
                    *point.get_z_depth_mut() = polygon_points_z_depth;
                    *point.get_color_mut() = Some(Color::random());
                    draw_buffer[point] = Color::from_rgb(255, 0, 0);
                    points.push(point);
                }
            }
        } else {
            is_mouse_pressed = false;
        }

        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) && points.len() >= POLYGON_SIZE
        {
            *points[0].get_color_mut() = Some(Color::from_rgb(255, 0, 0));
            *points[1].get_color_mut() = Some(Color::from_rgb(0, 255, 0));
            *points[2].get_color_mut() = Some(Color::from_rgb(0, 0, 255));
            let polygon = Polygon::<POLYGON_SIZE>::try_from(points.clone()).unwrap();
            points = points.into_iter().skip(POLYGON_SIZE).collect();

            fill_triangle(
                &polygon,
                &mut draw_buffer,
                &afro_obj.texture,
                None,
                light_dir,
                look_dir,
                None,
            );
        }

        if window.is_key_pressed(Key::C, minifb::KeyRepeat::No) {
            draw_buffer.clean();
            draw_buffer.get_z_buffer_mut().clean_with(&i32::MIN);
        }

        if let Some((scroll_x, scroll_y)) = window.get_scroll_wheel() {
            if window.is_key_down(Key::LeftShift) {
                let diff = -scroll_y / 100.0;
                cam_distance = (cam_distance + diff).max(0.85);
            } else {
                polygon_points_z_depth += (scroll_y * 10.0) as i32;
            }
        }

        window
            .update_with_buffer(
                draw_buffer.get_buffer_as_u32_ref(),
                draw_buffer.get_width(),
                draw_buffer.get_height(),
            )
            .unwrap();

        let end = Instant::now();
        let elapsed = (end - start).as_secs_f32();

        window.set_title(&format!(
            "{:.1?} FPS, depth: {}, θ: {}, φ: {}, r: {}",
            1.0 / elapsed,
            polygon_points_z_depth,
            cam_angle_theta,
            cam_angle_phi,
            from.distance(to)
        ));

        t += elapsed;
        if spin_light {
            light_spin_t += elapsed;
        }
    }

    Ok(())
}
