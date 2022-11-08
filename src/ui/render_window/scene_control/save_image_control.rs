use image::{DynamicImage, ImageFormat, ImageResult, RgbaImage};
use minifb::{Key, KeyRepeat, Window};

use crate::visual::drawing_buffer::DrawingBuffer;

const OUTPUT_FILE: &'static str = "output";
const OUTPUT_FORMAT: ImageFormat = ImageFormat::Bmp;

pub fn handle_image_save_controls(
    window: &Window,
    draw_buffer: &DrawingBuffer,
) -> ImageResult<bool> {
    if window.is_key_down(Key::LeftCtrl) && window.is_key_pressed(Key::S, KeyRepeat::No) {
        let output_file_path = format!("./{}.{}", OUTPUT_FILE, OUTPUT_FORMAT.extensions_str()[0]);
        println!("Saving image to \"{}\"", output_file_path);
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

        image.save_with_format(output_file_path, OUTPUT_FORMAT)?;
        Ok(true)
    } else {
        Ok(false)
    }
}
