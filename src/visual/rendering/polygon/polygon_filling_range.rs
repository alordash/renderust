use std::ops::Range;

use crate::math::interpolation::Interpolator;

use super::polygon_interpolation_values::PolygonInterpolationValues;

pub struct PolygonFillingRange<'a> {
    pub range: Range<isize>,
    pub interpolators: Vec<&'a (Interpolator<isize>, (PolygonInterpolationValues, PolygonInterpolationValues))>
}

impl<'a> Iterator for PolygonFillingRange<'a> {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        self.range.next()
    }
}