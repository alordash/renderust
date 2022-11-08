use crate::math::geometry::rect_size::RectSize;

use super::plane_buffer::PlaneBuffer;

impl<T: Default + Copy> PlaneBuffer<T> {
    pub fn resize(&mut self, new_size: RectSize) {
        let new_len = new_size.width * new_size.height;
        let new_value = T::default();

        let mut filling_range: &mut dyn Iterator<Item = _> = &mut (0..self.get_height()).rev();
        let mut straight_range = 0..self.get_height();

        if new_len > self.get_buffer().len() {
            self.get_buffer_mut().resize(new_len, new_value);
        } else if new_len < self.get_buffer().len() {
            filling_range = &mut straight_range;
        }
        if self.get_width() != new_size.width {
            let old_width = self.get_width();
            let new_width = new_size.width;
            unsafe {
                let buff_ptr = self.get_buffer_mut().as_mut_ptr();
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
        if new_len < self.get_buffer().len() {
            self.get_buffer_mut().resize(new_len, new_value);
        }
    }
}
