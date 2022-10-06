use std::{
    mem::size_of,
    ops::{Deref, Index, IndexMut, DerefMut},
    slice,
};

use crate::geometry::{primitives::point::Point, rect_size::RectSize};

use super::color::Color;

#[derive(Debug, Clone)]
pub struct PlaneBuffer<T> {
    size: RectSize,
    pub buffer: Vec<T>,
}

pub struct DrawBuffer(pub PlaneBuffer<Color>, pub PlaneBuffer<isize>);

impl Deref for DrawBuffer {
    type Target = PlaneBuffer<Color>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DrawBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DrawBuffer {
    pub fn new(
        width: usize,
        height: usize,
        create_option: PlaneBufferCreateOption<Color>,
    ) -> DrawBuffer {
        let size = width * height;
        let rect_size = RectSize { width, height };
        DrawBuffer(
            PlaneBuffer::<Color> {
                size: rect_size,
                buffer: match create_option {
                    PlaneBufferCreateOption::BLANK => vec![Color::default(); size],
                    PlaneBufferCreateOption::FILL(f) => (0..size).map(f).collect(),
                    _ => vec![Color::default(); size],
                },
            },
            PlaneBuffer {
                size: rect_size,
                buffer: vec![isize::MIN; size],
            },
        )
    }
}

#[non_exhaustive]
pub enum PlaneBufferCreateOption<T> {
    BLANK,
    FILL(fn(_: usize) -> T),
}

impl<T: Default + Copy> PlaneBuffer<T> {
    pub fn clean(&mut self) {
        self.buffer.fill(T::default());
    }

    pub fn clean_with(&mut self, filling_value: &T) {
        self.buffer.fill(*filling_value);
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
                PlaneBufferCreateOption::BLANK => vec![T::default(); size],
                PlaneBufferCreateOption::FILL(f) => (0..size).map(f).collect(),
                _ => vec![T::default(); size],
            },
        }
    }

    fn resize(&mut self, new_size: RectSize) {
        let new_len = new_size.width * new_size.height;
        let new_value = T::default();

        let mut filling_range: &mut dyn Iterator<Item = _> = &mut (0..self.size.height).rev();
        let mut straight_range = (0..self.size.height);

        if new_len > self.buffer.len() {
            self.buffer.resize(new_len, new_value);
        } else if new_len < self.buffer.len() {
            filling_range = &mut straight_range;
        }
        if self.size.width != new_size.width {
            let old_width = self.size.width;
            let new_width = new_size.width;
            unsafe {
                println!("Refilling");
                let buff_ptr = self.buffer.as_mut_ptr();
                for i in filling_range {
                    let dst_offset = new_width * i;
                    let src_offset = old_width * i;
                    let dst = buff_ptr.add(dst_offset);
                    let src = buff_ptr.add(src_offset);
                    let count = old_width.min(new_width);
                    std::ptr::copy_nonoverlapping(src, dst, count);
                    for j in dst_offset + old_width..dst_offset + new_width {
                        *buff_ptr.add(j) = new_value;
                    }
                }
            }
        }
        if new_len < self.buffer.len() {
            self.buffer.resize(new_len, new_value);
        }
    }

    pub fn get_width(&self) -> usize {
        self.size.width
    }

    pub fn set_width(&mut self, width: usize) {
        self.resize(RectSize {
            width,
            height: self.size.height,
        });
        self.size.width = width;
    }

    pub fn get_height(&self) -> usize {
        self.size.height
    }

    pub fn set_height(&mut self, height: usize) {
        self.resize(RectSize {
            width: self.size.width,
            height,
        });
        self.size.height = height;
    }

    pub fn get_size(&self) -> RectSize {
        self.size
    }

    pub fn set_size(&mut self, size: RectSize) {
        self.resize(size);
        self.size = size;
    }

    pub fn get_buffer_ref(&self) -> &Vec<T> {
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

impl<T> Index<(usize, usize)> for PlaneBuffer<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        unsafe {
            self.buffer
                .get_unchecked(index.0 + (self.size.height - index.1 - 1) * self.size.width)
        }
    }
}

impl<T> IndexMut<(usize, usize)> for PlaneBuffer<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        unsafe {
            self.buffer
                .get_unchecked_mut(index.0 + (self.size.height - index.1 - 1) * self.size.width)
        }
    }
}

impl<T> Index<Point> for PlaneBuffer<T> {
    type Output = T;
    fn index(&self, index: Point) -> &Self::Output {
        <Self as Index<(usize, usize)>>::index(self, (index.x() as usize, index.y() as usize))
    }
}

impl<T> IndexMut<Point> for PlaneBuffer<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        <Self as IndexMut<(usize, usize)>>::index_mut(
            self,
            (index.x() as usize, index.y() as usize),
        )
    }
}
