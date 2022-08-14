mod drawin;
mod geometry;

use std::time::Instant;

use drawin::{color::Color, draw_buffer::*, drawable::Drawable};
use geometry::{discrete_line::DiscreteLine, discrete_point::DiscretePoint, rect_size::RectSize};
use minifb::{Key, ScaleMode, Window, WindowOptions};
use rand::prelude::*;

const BUFFER_WIDTH: usize = 100;
const BUFFER_HEIGHT: usize = 100;

const WINDOW_WIDTH: usize = 400;
const WINDOW_HEIGHT: usize = 400;

const WIDTH_SCALE: f32 = WINDOW_WIDTH as f32 / BUFFER_WIDTH as f32;
const HEIGHT_SCALE: f32 = WINDOW_HEIGHT as f32 / BUFFER_HEIGHT as f32;

fn main() {
    // Allocate the output buffer.
    let mut draw_buffer =
        DrawBuffer::new(BUFFER_WIDTH, BUFFER_HEIGHT, DrawBufferCreateOption::BLANK);

    let mut window = Window::new(
        "Press ESC to exit",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open Window");

    let mut points: Vec<DiscretePoint> = Vec::new();
    let mut is_mouse_pressed = false;

    println!("W: {}, H: {}", WIDTH_SCALE, HEIGHT_SCALE);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let start = Instant::now();

        // let new_size: RectSize = window.get_size().into();
        // if draw_buffer.get_size() != new_size {
        //     println!("Resizing: {:?} -> {:?}", draw_buffer.get_size(), new_size);
        //     draw_buffer.set_size(new_size);
        // }

        if window.get_mouse_down(minifb::MouseButton::Left) {
            if !is_mouse_pressed {
                is_mouse_pressed = true;
                if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
                    let point: DiscretePoint =
                        ((x / WIDTH_SCALE) as isize, (y / HEIGHT_SCALE) as isize).into();
                    points.push(point);
                }
            }
        } else {
            is_mouse_pressed = false;
        }

        if points.len() > 1 {
            let len = points.len();
            let even_len = if len % 2 == 0 { len } else { len - 1 };
            let iterating_points: Vec<DiscretePoint> = points.drain(0..even_len).collect();
            for points_chunk in iterating_points.chunks_exact(2) {
                unsafe {
                    let (p1, p2) = (points_chunk.get_unchecked(0), points_chunk.get_unchecked(1));
                    let line = DiscreteLine {
                        begin: *p1,
                        end: *p2,
                    };
                    line.draw(&mut draw_buffer, &Color::random());
                }
            }
            println!("Previous len: {}, new: {}", len, points.len());
        }

        window
            .update_with_buffer(
                draw_buffer.get_buffer_as_u32_ref(),
                draw_buffer.get_width(),
                draw_buffer.get_height(),
            )
            .unwrap();

        let end = Instant::now();

        window.set_title(&format!("{:.1?} FPS", 1.0 / (end - start).as_secs_f32()));
    }
}
