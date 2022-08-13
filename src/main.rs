mod draw_buffer;
mod geometry;

use draw_buffer::*;
use geometry::rect_size::RectSize;
use minifb::{Key, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 300;
const HEIGHT: usize = 300;

fn main() {
    // Allocate the output buffer.
    let mut draw_buffer = DrawBuffer::new(WIDTH, HEIGHT, DrawBufferCreateOption::RANDOM);

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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let new_size: RectSize = window.get_size().into();
        if draw_buffer.get_size() != new_size {
            println!("Resizing: {:?} -> {:?}", draw_buffer.get_size(), new_size);
            draw_buffer.set_size(new_size);
        }

        window
            .update_with_buffer(
                draw_buffer.get_buffer_as_u32_ref(),
                draw_buffer.get_width(),
                draw_buffer.get_height(),
            )
            .unwrap();
    }
}
