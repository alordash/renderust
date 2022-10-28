#![feature(min_specialization)]

use glam::{Vec3, Mat4};
use math::{geometry::{
    primitives::{point::Point2D, polygon::Polygon},
    rect_size::RectSize,
}, spherical_coordinate_system::spherical_to_cartesian};

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
        view_matrix::create_view_matrix_and_look_dir,
        wavefront_obj::wavefront_obj_rendering::{render_wavefront_grid, render_wavefront_mesh},
    },
};
use wavefront::wavefront_obj::WavefrontObj;

const BUFFER_WIDTH: usize = 1000;
const BUFFER_HEIGHT: usize = 1000;

const WINDOW_WIDTH: usize = 1000;
const WINDOW_HEIGHT: usize = 1000;

const WAVEFRONT_SOURCE_PATH: &'static str = "./resources/african_head.obj";
const TEXTURE_SOURCE_PATH: &'static str = "./resources/african_head_diffuse.tga";
const NORMAL_MAP_SOURCE_PATH: &'static str = "./resources/african_head_nm_tangent.tga";

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
    let color_step = 30.5;

    let mut light_dir = Vec3::new(0.0, 0.0, 1.0).normalize();
    let look_dir = Vec3::new(0.0, 0.0, 1.0).normalize();

    let mut polygon_points_z_depth = 5000i32;

    let wavefront_obj_file = File::open(WAVEFRONT_SOURCE_PATH)
        .map_err(|e| format!("Error opening model file: {:?}", e))?;
    let texture_file = File::open(TEXTURE_SOURCE_PATH)
        .map_err(|e| format!("Error opening texture file: {:?}", e))?;
    let normal_map_file = File::open(NORMAL_MAP_SOURCE_PATH)
        .map_err(|e| format!("Error opening normal map file: {:?}", e))?;
    let wavefront_obj =
        WavefrontObj::from_file(&wavefront_obj_file, &texture_file, &normal_map_file)
            .map_err(|e| format!("Error parsing file: {:?}", e))?;

    let mut points: Vec<Point2D> = Vec::new();

    let mut from = Vec3::new(0.0, 0.0, 1.0);
    let to = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::Y;

    let mut cam_angle_theta = 0.0;
    let mut cam_angle_phi = 0.0;
    let mut cam_distance = 1.0;

    let (mut view_matrix, mut look_dir) =
        create_view_matrix_and_look_dir(from, to, up);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let start = Instant::now();

        let new_size: RectSize = window.get_size().into();
        if draw_buffer.get_size() != new_size {
            width_scale = new_size.width as f32 / BUFFER_WIDTH as f32;
            height_scale = new_size.height as f32 / BUFFER_HEIGHT as f32;
        }

        let passed_hue = (t * color_step) as u16 % 360_u16;

        let color = Color::from_hsv(passed_hue, 1.0, 1.0);

        if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
            let window_size = RectSize::from(window.get_size());

            cam_angle_theta = (y / window_size.height as f32) * std::f32::consts::PI;
            cam_angle_phi = ((x - window_size.width as f32 / 2.0) / window_size.width as f32) * std::f32::consts::PI * 2.0;

            from = spherical_to_cartesian(cam_angle_theta, cam_angle_phi, cam_distance).into();
            
            // light_dir = Vec3::new(
            //     (x - window_size.width as f32 / 2.0),
            //     0.0,
            //     (y - window_size.height as f32 / 2.0),
            // )
            // .normalize();
            // from = light_dir;
            (view_matrix, _) = create_view_matrix_and_look_dir(from, to, up);
            // look_dir.x = -look_dir.x;
            // look_dir.y = -look_dir.y;
        }
        draw_buffer.get_z_buffer_mut().clean_with(&i32::MIN);
        draw_buffer.clean();
        
        let mut projection = Mat4::IDENTITY;
        projection.col_mut(2)[3] = -1.0 / from.distance(to);

        render_wavefront_mesh(
            &wavefront_obj,
            &mut draw_buffer,
            look_dir,
            look_dir,
            None,
            window.is_key_down(Key::LeftShift),
            projection,
            view_matrix,
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
            // fill_polygon(
            //     &polygon,
            //     &mut draw_buffer,
            //     &wavefront_obj.texture,
            //     None,
            //     Vec3::new(1.0, 0.0, 0.0),
            //     look_dir,
            //     None,
            // );

            fill_triangle(
                &polygon,
                &mut draw_buffer,
                &wavefront_obj.texture,
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

        window.set_title(&format!(
            "{:.1?} FPS, depth: {}, θ: {}, φ: {}, r: {}",
            1.0 / (end - start).as_secs_f32(),
            polygon_points_z_depth,
            cam_angle_theta, 
            cam_angle_phi,
            from.distance(to)
        ));

        t += time_step;
    }

    Ok(())
}
