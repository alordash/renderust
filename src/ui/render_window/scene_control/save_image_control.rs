use image::{DynamicImage, ImageResult, RgbaImage};
use minifb::{Key, KeyRepeat, Window};

use crate::visual::drawing_buffer::DrawingBuffer;

pub fn handle_image_save_controls(window: &Window, draw_buffer: &DrawingBuffer) -> ImageResult<()> {
    if window.is_key_down(Key::LeftCtrl) && window.is_key_pressed(Key::S, KeyRepeat::No) {
        println!("Saving image");
        let image = RgbaImage::from_vec(
            draw_buffer.get_width() as u32,
            draw_buffer.get_height() as u32,
            draw_buffer
                .get_buffer()
                .iter()
                .flat_map(|color| [color.r, color.g, color.b, color.alpha])
                .collect(),
        )
        .unwrap();

        let image = DynamicImage::ImageRgba8(image);

        image.save_with_format("./output.bmp", image::ImageFormat::Bmp)?;
    }
    Ok(())
}
