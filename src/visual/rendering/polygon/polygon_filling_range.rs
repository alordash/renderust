use std::ops::Range;

use crate::math::interpolation::Interpolator;

use super::polygon_interpolation_values::PolygonInterpolationValues;

pub struct PolygonFillingRange<'a> {
    pub range: Range<i32>,
    pub interpolators: Vec<&'a (Interpolator<i32>, (PolygonInterpolationValues, PolygonInterpolationValues))>
}

impl<'a> Iterator for PolygonFillingRange<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.range.next()
    }
}