use std::ops::{Index, IndexMut};

use super::plane_buffer::PlaneBuffer;

impl<T> Index<(usize, usize)> for PlaneBuffer<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        unsafe {
            self.get_buffer()
                .get_unchecked(index.0 + (self.get_height() - index.1 - 1) * self.get_width())
        }
    }
}

impl<T> IndexMut<(usize, usize)> for PlaneBuffer<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        unsafe {
            let (width, height) = (self.get_width(), self.get_height());
            self.get_buffer_mut()
                .get_unchecked_mut(index.0 + (height - index.1 - 1) * width)
        }
    }
}
