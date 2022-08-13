use std::{
    mem::size_of,
    ops::{Index, IndexMut},
    slice,
};

use rand::prelude::*;

use crate::geometry::DiscretePoint;

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

#[derive(Debug)]
pub struct DrawBuffer {
    width: usize,
    height: usize,
    buffer: Vec<Color>,
}

#[non_exhaustive]
pub enum DrawBufferCreateOption {
    BLANK,
    RANDOM,
}

impl DrawBuffer {
    pub fn new(width: usize, height: usize, create_option: DrawBufferCreateOption) -> DrawBuffer {
        let size = width * height;
        DrawBuffer {
            width,
            height,
            buffer: match create_option {
                DrawBufferCreateOption::BLANK => vec![Color::default(); size],
                DrawBufferCreateOption::RANDOM => (0..size).map(|_| Color::random()).collect(),
                _ => vec![Color::default(); size],
            },
        }
    }

    fn resize(&mut self) {
        let new_size = self.width * self.height;
        self.buffer.resize(new_size, Color::random());
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn set_width(&mut self, width: usize) {
        self.width = width;
        self.resize();
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn set_height(&mut self, height: usize) {
        self.height = height;
        self.resize();
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn set_size(&mut self, size: (usize, usize)) {
        (self.width, self.height) = size;
        self.resize();
    }

    pub fn get_buffer_ref(&self) -> &Vec<Color> {
        &self.buffer
    }

    pub fn get_buffer_as_u32_ref(&self) -> &[u32] {
        unsafe {
            slice::from_raw_parts(
                self.buffer.as_ptr() as *const u32,
                self.buffer.len() * size_of::<Color>() / size_of::<u32>(),
            )
        }
    }
}

impl Index<(usize, usize)> for DrawBuffer {
    type Output = Color;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        unsafe { self.buffer.get_unchecked(index.0 + index.1 * self.width) }
    }
}

impl IndexMut<(usize, usize)> for DrawBuffer {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        unsafe {
            self.buffer
                .get_unchecked_mut(index.0 + index.1 * self.width)
        }
    }
}

impl Index<DiscretePoint> for DrawBuffer {
    type Output = Color;
    fn index(&self, index: DiscretePoint) -> &Self::Output {
        <Self as Index<(usize, usize)>>::index(self, (index.x, index.y))
    }
}

impl IndexMut<DiscretePoint> for DrawBuffer {
    fn index_mut(&mut self, index: DiscretePoint) -> &mut Self::Output {
        <Self as IndexMut<(usize, usize)>>::index_mut(self, (index.x, index.y))
    }
}
