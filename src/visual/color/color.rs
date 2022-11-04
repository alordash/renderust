use std::ops::Mul;

use glam::Vec3A;
use image::Rgba;
use num::{Num, NumCast};
use rand::prelude::*;

use crate::{derive_self_wrapping_add, derive_self_wrapping_sub};

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    pub fn inverted(&self) -> Color {
        let mut color = *self;
        color.invert();
        color
    }

    pub fn apply_intensity(self, intensities: Vec3A) -> Color {
        let r = (self.r as f32 * intensities.x) as u8;
        let g = (self.g as f32 * intensities.y) as u8;
        let b = (self.b as f32 * intensities.z) as u8;
        Color::from_rgb_with_alpha(r, g, b, self.alpha)
    }
}

impl From<Rgba<u8>> for Color {
    fn from(rgba: Rgba<u8>) -> Self {
        let (r, g, b, alpha) = (rgba.0[0], rgba.0[1], rgba.0[2], rgba.0[3]);
        Color { b, g, r, alpha }
    }
}

impl<T: Num + Copy + NumCast> Mul<T> for Color {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let r = <u8 as NumCast>::from(T::from(self.r).unwrap() * rhs).unwrap();
        let g = <u8 as NumCast>::from(T::from(self.g).unwrap() * rhs).unwrap();
        let b = <u8 as NumCast>::from(T::from(self.b).unwrap() * rhs).unwrap();
        Color::from_rgb_with_alpha(r, g, b, self.alpha)
    }
}

derive_self_wrapping_add!(Color, r, g, b, alpha);
derive_self_wrapping_sub!(Color, r, g, b, alpha);
