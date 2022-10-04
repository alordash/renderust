use std::ops::Mul;

use num::{Num, NumCast};
use rand::prelude::*;

use crate::{derive_self_add, derive_self_sub};

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
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

    pub fn invert(&mut self) {
        self.r = u8::MAX - self.r;
        self.g = u8::MAX - self.g;
        self.b = u8::MAX - self.b;
    }

    pub fn copy_invert(&self) -> Color {
        let mut color = *self;
        color.invert();
        color
    }
}

impl<T: Num + Copy + NumCast> Mul<T> for Color {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let r = <u8 as NumCast>::from(T::from(self.r).unwrap() * rhs).unwrap();
        let g = <u8 as NumCast>::from(T::from(self.g).unwrap() * rhs).unwrap();
        let b = <u8 as NumCast>::from(T::from(self.b).unwrap() * rhs).unwrap();
        Color {
            r,
            g,
            b,
            alpha: self.alpha,
        }
    }
}

// (b - a) * t + a

impl Color {
    pub fn mul_div<T: Num + Copy + NumCast>(self, mul: T, div: T) -> Color {
        let r = <u8 as NumCast>::from(T::from(self.r).unwrap() * mul / div).unwrap();
        let g = <u8 as NumCast>::from(T::from(self.g).unwrap() * mul / div).unwrap();
        let b = <u8 as NumCast>::from(T::from(self.b).unwrap() * mul / div).unwrap();
        Color {
            r,
            g,
            b,
            alpha: self.alpha,
        }
    }

    pub fn mix(self, rhs: Color, t: i32, max: i32) -> Color {
        let r = (((rhs.r as i32 - self.r as i32) * t / max) as u8).wrapping_add(self.r);
        let g = (((rhs.g as i32 - self.g as i32) * t / max) as u8).wrapping_add(self.g);
        let b = (((rhs.b as i32 - self.b as i32) * t / max) as u8).wrapping_add(self.b);
        Color { r, g, b, ..self }
    }
}

// impl Mul<isize> for Color {
//     type Output = Color;
//     fn mul(self, rhs: isize) -> Self::Output {
//         Color {
//             r: ((self.r as isize) * rhs) as u8,
//             g: ((self.g as isize) * rhs) as u8,
//             b: ((self.b as isize) * rhs) as u8,
//             ..self
//         }
//     }
// }

derive_self_add!(Color, r, g, b, alpha);
derive_self_sub!(Color, r, g, b, alpha);
