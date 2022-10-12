use std::ops::Range;

use crate::{math::interpolation::Interpolator, visual::rendering::interpolation_values::InterpolationValues};

pub struct PolygonFillingRange<'a> {
    pub range: Range<i32>,
    pub interpolators: Vec<&'a (Interpolator<i32>, (InterpolationValues, InterpolationValues))>
}

impl<'a> Iterator for PolygonFillingRange<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.range.next()
    }
}