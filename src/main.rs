#![feature(min_specialization)]

use math::{
    geometry::{
        primitives::{point::Point2D, polygon::Polygon},
        rect_size::RectSize,
    },
    vector::{common_vectors::vec3f::Vec3f, linear_algebra::LinAlgOperations},
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

    let mut light_dir = Vec3f::new([0.0, 0.0, 1.0]).normalized();
    let look_dir = Vec3f::new([0.0, 0.0, 1.0]).normalized();

    let mut polygon_points_z_depth = 0isize;

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
            light_dir = Vec3f::new([
                (x - window_size.width as f32 / 2.0),
                0.0,
                (y - window_size.height as f32 / 2.0),
            ])
            .normalized();
        }
        let prev_z_buffer = draw_buffer.get_z_buffer().clone();
        render_wavefront_mesh(
            &wavefront_obj,
            &mut draw_buffer,
            light_dir,
            look_dir,
            None,
            window.get_mouse_down(minifb::MouseButton::Right),
        );
        *draw_buffer.get_z_buffer_mut() = prev_z_buffer;

        if window.get_mouse_down(minifb::MouseButton::Left) {
            if !is_mouse_pressed {
                is_mouse_pressed = true;
                if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
                    let y = new_size.height as f32 - y - 1.0;
                    let mut point =
                        Point2D::from((x / width_scale) as isize, (y / height_scale) as isize);
                    *point.get_normal_mut() = Vec3f::new([1.0, 0.0, 0.0]);
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
            let polygon = Polygon::<POLYGON_SIZE>::try_from(points.clone()).unwrap();
            points = points.into_iter().skip(POLYGON_SIZE).collect();
            fill_polygon(
                &polygon,
                &mut draw_buffer,
                &wavefront_obj.texture,
                Some(&wavefront_obj.normal_map),
                Vec3f::new([1.0, 0.0, 0.0]),
                look_dir,
                None,
            );
        }

        if window.is_key_pressed(Key::C, minifb::KeyRepeat::No) {
            draw_buffer.clean();
            draw_buffer.get_z_buffer_mut().clean_with(&isize::MIN);
        }

        if let Some((scroll_x, scroll_y)) = window.get_scroll_wheel() {
            polygon_points_z_depth += (scroll_y * 10.0) as isize;
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
            "{:.1?} FPS, depth: {}",
            1.0 / (end - start).as_secs_f32(),
            polygon_points_z_depth
        ));

        t += time_step;
    }

    Ok(())
}
