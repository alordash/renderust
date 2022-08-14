mod drawin;
mod geometry;

use std::time::Instant;

use drawin::{color::Color, draw_buffer::*};
use geometry::{discrete_point::DiscretePoint, rect_size::RectSize};
use minifb::{Key, ScaleMode, Window, WindowOptions};
use rand::prelude::*;

const WIDTH: usize = 150;
const HEIGHT: usize = 30;
const DEFAULT_AREA: usize = WIDTH * HEIGHT;

fn main() {
    // Allocate the output buffer.
    let mut draw_buffer = DrawBuffer::new(WIDTH, HEIGHT, DrawBufferCreateOption::BLANK);

    let mut window = Window::new(
        "Press ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::UpperLeft,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open Window");

    let mut rng = rand::thread_rng();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let start = Instant::now();

        let new_size: RectSize = window.get_size().into();
        if draw_buffer.get_size() != new_size {
            println!("Resizing: {:?} -> {:?}", draw_buffer.get_size(), new_size);
            draw_buffer.set_size(new_size);
        }

        let RectSize { width, height } = draw_buffer.get_size();
        
        for _ in 0..DEFAULT_AREA {
            let point = DiscretePoint {
                x: rng.gen_range(0..width),
                y: rng.gen_range(0..height),
            };
            let new_color = Color::random();

            draw_buffer[point] = new_color;
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
