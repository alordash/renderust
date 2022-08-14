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

    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { b, g, r, alpha: 0 }
    }

    pub fn new_with_alpha(r: u8, g: u8, b: u8, alpha: u8) -> Color {
        Color { b, g, r, alpha }
    }
}
