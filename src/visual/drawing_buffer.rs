use std::ops::{Deref, DerefMut};

use crate::plane_buffer::plane_buffer::{PlaneBuffer, PlaneBufferCreateOption};

use super::color::color::Color;

pub struct DrawingBuffer(PlaneBuffer<Color>, PlaneBuffer<f32>);

impl Deref for DrawingBuffer {
    type Target = PlaneBuffer<Color>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DrawingBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DrawingBuffer {
    pub fn new(
        width: usize,
        height: usize,
        create_option: PlaneBufferCreateOption<Color>,
    ) -> DrawingBuffer {
        DrawingBuffer(
            PlaneBuffer::new(width, height, create_option),
            PlaneBuffer::new(width, height, PlaneBufferCreateOption::Fill(|_| f32::MIN)),
        )
    }

    pub fn get_z_buffer(&self) -> &PlaneBuffer<f32> {
        &self.1
    }

    pub fn get_z_buffer_mut(&mut self) -> &mut PlaneBuffer<f32> {
        &mut self.1
    }
}
