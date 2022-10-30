use std::{mem::size_of, slice};

use crate::math::geometry::rect_size::RectSize;

pub enum PlaneBufferCreateOption<T> {
    Blank,
    Fill(fn(_: usize) -> T),
    RawSource(Vec<T>)
}

#[derive(Debug, Clone)]
pub struct PlaneBuffer<T> {
    size: RectSize,
    buffer: Vec<T>,
}

impl<T> PlaneBuffer<T> {
    pub fn clean(&mut self)
    where
        T: Default + Clone,
    {
        self.buffer.fill(T::default());
    }

    pub fn clean_with(&mut self, filling_value: &T)
    where
        T: Copy,
    {
        self.buffer.fill(*filling_value);
    }

    pub fn get_width(&self) -> usize {
        self.size.width
    }

    pub fn get_height(&self) -> usize {
        self.size.height
    }

    pub fn get_size(&self) -> RectSize {
        self.size
    }

    pub fn get_buffer(&self) -> &Vec<T> {
        &self.buffer
    }

    pub fn get_buffer_mut(&mut self) -> &mut Vec<T> {
        &mut self.buffer
    }

    pub fn get_buffer_as_u32_ref(&self) -> &[u32] {
        unsafe {
            slice::from_raw_parts(
                self.buffer.as_ptr() as *const u32,
                self.buffer.len() * size_of::<T>() / size_of::<u32>(),
            )
        }
    }

    pub fn contains(&self, x: usize, y: usize) -> bool {
        x < self.size.width && y < self.size.height
    }
}

impl<T: Default + Copy> PlaneBuffer<T> {
    pub fn new(
        width: usize,
        height: usize,
        create_option: PlaneBufferCreateOption<T>,
    ) -> PlaneBuffer<T> {
        let size = width * height;
        PlaneBuffer::<T> {
            size: RectSize { width, height },
            buffer: match create_option {
                PlaneBufferCreateOption::Blank => vec![T::default(); size],
                PlaneBufferCreateOption::Fill(f) => (0..size).map(f).collect(),
                PlaneBufferCreateOption::RawSource(source) => source,
            },
        }
    }

    pub fn set_width(&mut self, width: usize) {
        self.resize(RectSize {
            width,
            height: self.size.height,
        });
        self.size.width = width;
    }

    pub fn set_height(&mut self, height: usize) {
        self.resize(RectSize {
            width: self.size.width,
            height,
        });
        self.size.height = height;
    }

    pub fn set_size(&mut self, size: RectSize) {
        self.resize(size);
        self.size = size;
    }
}
