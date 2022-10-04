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

const BUFFER_WIDTH: usize = 1000;
const BUFFER_HEIGHT: usize = 1000;

const WINDOW_WIDTH: usize = 1000;
const WINDOW_HEIGHT: usize = 1000;

const WAVEFRONT_SOURCE_PATH: &'static str = "./resources/african_head.obj";

const POLYGON_SIZE: usize = 1000;
const POLYGON_COUNT: usize = 100;

fn gen_points(width: usize, height: usize) -> Vec<Point> {
    let mut rng = thread_rng();
    (0..POLYGON_SIZE)
        .map(|_| {
            Point::new_with_color(
                rng.gen_range(0..width as isize),
                rng.gen_range(0..height as isize),
                Color::random()
            )
        })
        .collect()
}

fn main() -> Result<(), String> {
    // Allocate the output buffer.
    let mut draw_buffer =
        DrawBuffer::new(BUFFER_WIDTH, BUFFER_HEIGHT, DrawBufferCreateOption::BLANK);

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

    let color_step = 3.5;

    let wavefront_obj_file =
        File::open(WAVEFRONT_SOURCE_PATH).map_err(|e| format!("Error opening file: {:?}", e))?;
    let wavefront_obj = WavefrontObj::from_file(&wavefront_obj_file)
        .map_err(|e| format!("Error parsing file: {:?}", e))?;

    // wavefront_obj.fill(&mut draw_buffer, &Color::random());
    // wavefront_obj.draw(&mut draw_buffer, &Color::from_rgb(255, 255, 255));

    let polygons: Vec<_> = (0..POLYGON_COUNT)
        .map(|_| {
            Polygon::<POLYGON_SIZE>::from(gen_points(
                draw_buffer.get_width(),
                draw_buffer.get_height(),
            ))
        })
        .collect();

    let mut rng = thread_rng();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let start = Instant::now();

        let new_size: RectSize = window.get_size().into();
        if draw_buffer.get_size() != new_size {
            width_scale = new_size.width as f32 / BUFFER_WIDTH as f32;
            height_scale = new_size.height as f32 / BUFFER_HEIGHT as f32;
        }

        if window.get_mouse_down(minifb::MouseButton::Left) {
            // wavefront_obj.draw(&mut draw_buffer, &Color::from_rgb(255, 255, 255));
            if !is_mouse_pressed {
                is_mouse_pressed = true;
                if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
                    let y = new_size.height as f32 - y - 1.0;
                    let mut point: Point =
                        ((x / width_scale) as isize, (y / height_scale) as isize).into();
                    point.color = Color::random();
                    draw_buffer[point] = Color::from_rgb(255, 0, 0);
                    points.push(point);
                }
            }
        } else {
            is_mouse_pressed = false;
        }

        // let size = draw_buffer.get_size();
        // let half_width = size.width as isize / 2;
        // let half_height = size.height as isize / 2;

        // let r = half_width.min(half_height) as f32;

        // let first_point = DiscretePoint {
        //     x: (r * angle.to_radians().cos()) as isize + half_width,
        //     y: (r * angle.to_radians().sin()) as isize + half_height,
        // };

        // // let second_point = DiscretePoint {
        // //     x: -(first_point.x - half_width as isize) + half_width,
        // //     y: -(first_point.y - half_height as isize) + half_height,
        // // };

        // let second_point = DiscretePoint {
        //     x: half_width,
        //     y: half_height,
        // };

        // points.push(first_point);
        // points.push(second_point);

        // // angle += (t.sin() + 0.25) * angle_step;
        // angle += angle_step;

        let passed_hue = (t * color_step) as u16 % 360_u16;

        let color = Color::from_hsv(passed_hue, 1.0, 1.0);

        // if points.len() > 1 {
        //     let len = points.len();
        //     let even_len = if len % 2 == 0 { len } else { len - 1 };
        //     let mut iterating_points: Vec<Point> = points.drain(0..even_len).collect();
        //     for points_chunk in iterating_points.chunks_exact_mut(2) {
        //         unsafe {
        //             // points_chunk.get_unchecked_mut(1).color.invert();
        //             points_chunk.get_unchecked_mut(1).color.invert();
        //             let (p1, p2) = (points_chunk.get_unchecked(0), points_chunk.get_unchecked(1));
        //             let line = Line {
        //                 begin: *p1,
        //                 end: *p2,
        //             };
        //             // line.draw(&mut draw_buffer, &color);
        //             line.rough_draw(&mut draw_buffer);
        //         }
        //     }
        // }

        // if points.len() > 2 {
        //     let len = points.len();
        //     let dividable_len = (len / 3) * 3;
        //     let iterating_points: Vec<DiscretePoint> = points.drain(0..dividable_len).collect();
        //     for points_chunk in iterating_points.chunks_exact(3) {
        //         unsafe {
        //             let triangle_points: [DiscretePoint; 3] = points_chunk.try_into().unwrap();
        //             let triangle = DiscreteTriangle {
        //                 points: triangle_points,
        //             };
        //             triangle.draw(&mut draw_buffer, &color);
        //         }
        //     }
        // }

        if !window.is_key_down(Key::Space) {
            // let color = Color::random();

            // // points = gen_points(draw_buffer.get_width(), draw_buffer.get_height());
            // // let polygon = DiscretePolygon::<POLYGON_SIZE>::from(points.clone());
            let idx = rng.gen_range(0..POLYGON_COUNT);
            let polygon = &polygons[idx];
            polygon.fill(&mut draw_buffer, &color.copy_invert());
            // polygon.draw(&mut draw_buffer, &color);
        }

        // if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) && points.len() >= POLYGON_SIZE
        // {
        //     let polygon = Polygon::<POLYGON_SIZE>::from(points.clone());
        //     points = points.into_iter().skip(POLYGON_SIZE).collect();
        //     polygon.fill(&mut draw_buffer, &color.copy_invert());
        //     // polygon.draw(&mut draw_buffer, &color);
        // }

        if window.is_key_pressed(Key::C, minifb::KeyRepeat::No) {
            draw_buffer.clean();
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
            "({}) {:.1?} FPS",
            points.len(),
            1.0 / (end - start).as_secs_f32()
        ));

        t += time_step;
    }

    Ok(())
}
