use crate::drawin::drawable::Drawable;

use super::discrete_point::DiscretePoint;

pub struct DiscretePolygon<const N: usize> {
    pub points: [DiscretePoint; N],
}