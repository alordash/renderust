use rand::prelude::*;

#[derive(Default, Clone, Copy, Debug)]
#[repr(C)]
pub struct Color {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub alpha: u8,
}

impl Color {
    pub fn random() -> Color {
        let mut rng = rand::thread_rng();
        let range = u8::MIN..u8::MAX;
        Color {
            r: rng.gen_range(range.clone()),
            g: rng.gen_range(range.clone()),
            b: rng.gen_range(range.clone()),
            alpha: 255,
        }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color { b, g, r, alpha: 0 }
    }

    pub fn from_rgb_with_alpha(r: u8, g: u8, b: u8, alpha: u8) -> Color {
        Color { b, g, r, alpha }
    }

    pub fn from_hsv(h: u16, s: f32, v: f32) -> Color {
        let c = v * s as f32;
        let x = c * (1.0 - ((h as f32 / 60.0) % 2.0 - 1.0).abs()) as f32;
        let m = v - c;

        let (r0, g0, b0) = match h {
            0..=59 => (c, x, 0.0),
            60..=119 => (x, c, 0.0),
            120..=179 => (0.0, c, x),
            180..=239 => (0.0, x, c),
            240..=299 => (x, 0.0, c),
            300..=359 => (c, 0.0, x),
            _ => (0.0, 0.0, 0.0),
        };

        let (r, g, b): (u8, u8, u8) = (
            ((r0 + m) * 255.0) as u8,
            ((g0 + m) * 255.0) as u8,
            ((b0 + m) * 255.0) as u8,
        );

        Color::from_rgb(r, g, b)
    }
}
