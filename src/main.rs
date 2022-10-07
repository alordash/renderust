#![feature(min_specialization)]

mod discretization;
mod drawin;
mod geometry;
mod wavefront;

use std::{
    fs::File,
    ops::{RangeFull, RangeTo, RangeToInclusive},
    time::Instant,
};

use drawin::{color::Color, draw_buffer::*, drawable::Drawable};
use geometry::{
    primitives::{line::Line, point::Point, polygon::Polygon, polygons::triangle::Triangle},
    rect_size::RectSize,
};
use minifb::{Key, ScaleMode, Window, WindowOptions};
use rand::prelude::*;
use wavefront::wavefront_obj::WavefrontObj;

use image::GenericImage;

const BUFFER_WIDTH: usize = 1000;
const BUFFER_HEIGHT: usize = 1000;

const WINDOW_WIDTH: usize = 1000;
const WINDOW_HEIGHT: usize = 1000;

const WAVEFRONT_SOURCE_PATH: &'static str = "./resources/african_head.obj";
const TEXTURE_SOURCE_PATH: &'static str = "./resources/african_head_diffuse.tga";

const POLYGON_SIZE: usize = 3;
const POLYGON_COUNT: usize = 100;

fn gen_points(width: usize, height: usize) -> Vec<Point> {
    let mut rng = thread_rng();
    (0..POLYGON_SIZE)
        .map(|_| {
            Point::new_with_color(
                rng.gen_range(0..width as isize),
                rng.gen_range(0..height as isize),
                Color::random(),
            )
        })
        .collect()
}

// fn image_test() {
//     let img = image::open(TEXTURE_SOURCE_PATH).unwrap();
//     println!("dimensons: {:?}", img.dimensions());
//     println!("{:?}", img.color());
//     let a = img.get_pixel(5, 5);
// }

fn main() -> Result<(), String> {
    // Allocate the output buffer.
    let mut draw_buffer =
        DrawBuffer::new(BUFFER_WIDTH, BUFFER_HEIGHT, PlaneBufferCreateOption::BLANK);

    let mut window = Window::new(
        "Press ESC to exit",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::Stretch,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open Window");

    let mut width_scale = WINDOW_WIDTH as f32 / BUFFER_WIDTH as f32;
    let mut height_scale = WINDOW_HEIGHT as f32 / BUFFER_HEIGHT as f32;

    let mut points: Vec<Point> = Vec::new();
    let mut is_mouse_pressed = false;

    let mut angle: f32 = 0.0;
    let angle_step: f32 = 20.0_f32.to_radians();

    let mut t: f32 = 0.0;
    let time_step = 0.05;

    let color_step = 30.5;

    let wavefront_obj_file = File::open(WAVEFRONT_SOURCE_PATH)
        .map_err(|e| format!("Error opening model file: {:?}", e))?;
    let texture_file = File::open(TEXTURE_SOURCE_PATH)
        .map_err(|e| format!("Error opening texture file: {:?}", e))?;
    let wavefront_obj = WavefrontObj::from_file(&wavefront_obj_file, &texture_file)
        .map_err(|e| format!("Error parsing file: {:?}", e))?;

    wavefront_obj.fill(&mut draw_buffer, Some(&Color::random()));
    wavefront_obj.draw(&mut draw_buffer, Some(&Color::from_rgb(255, 255, 255)));

    let polygons: Vec<_> = (0..POLYGON_COUNT)
        .map(|_| {
            Polygon::<POLYGON_SIZE>::from(gen_points(
                draw_buffer.get_width(),
                draw_buffer.get_height(),
            ))
        })
        .collect();

    let mut rng = thread_rng();

    let mut rough = true;

    let mut polygon_depth = 0isize;

    // image_test();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let start = Instant::now();

        let new_size: RectSize = window.get_size().into();
        if draw_buffer.get_size() != new_size {
            width_scale = new_size.width as f32 / BUFFER_WIDTH as f32;
            height_scale = new_size.height as f32 / BUFFER_HEIGHT as f32;
        }

        let passed_hue = (t * color_step) as u16 % 360_u16;

        let color = Color::from_hsv(passed_hue, 1.0, 1.0);

        if window.get_mouse_down(minifb::MouseButton::Left) {
            // wavefront_obj.draw(&mut draw_buffer, &Color::from_rgb(255, 255, 255));
            if !is_mouse_pressed {
                is_mouse_pressed = true;
                if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
                    let y = new_size.height as f32 - y - 1.0;
                    let mut point = Point::new_with_z(
                        (x / width_scale) as isize,
                        (y / height_scale) as isize,
                        polygon_depth,
                    );
                    // point.color = Color::random();
                    point.color = color;
                    draw_buffer[point] = Color::from_rgb(255, 0, 0);
                    points.push(point);
                }
            }
        } else {
            is_mouse_pressed = false;
        }

        wavefront_obj.fill(&mut draw_buffer, Some(&color));

        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) && points.len() >= POLYGON_SIZE
        {
            while points.len() >= 2 * POLYGON_SIZE {
                points = points.into_iter().skip(POLYGON_SIZE).collect();
            }
            let polygon = Polygon::<POLYGON_SIZE>::from(points.clone());
            polygon.fill(&mut draw_buffer, None);
        }

        if window.is_key_pressed(Key::C, minifb::KeyRepeat::No) {
            draw_buffer.clean();
        }

        if let Some((scroll_x, scroll_y)) = window.get_scroll_wheel() {
            polygon_depth += (scroll_y * 10.0) as isize;
        }

        if window.is_key_pressed(Key::Backspace, minifb::KeyRepeat::No) {
            draw_buffer.clean();
            draw_buffer.1.clean_with(&isize::MIN);
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
            "({}) {:.1?} FPS, precise: {:?}, depth: {:?}",
            points.len(),
            1.0 / (end - start).as_secs_f32(),
            !rough,
            polygon_depth
        ));

        t += time_step;
    }

    Ok(())
}
