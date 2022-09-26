use crate::{derive_self_add, derive_self_sub};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DiscretePoint {
    pub x: isize,
    pub y: isize,
}

impl DiscretePoint {
    pub fn new(x: isize, y: isize) -> DiscretePoint {
        DiscretePoint { x, y }
    }
}

impl From<(isize, isize)> for DiscretePoint {
    fn from(source: (isize, isize)) -> Self {
        DiscretePoint {
            x: source.0,
            y: source.1,
        }
    }
}

derive_self_add!(DiscretePoint, x, y);
derive_self_sub!(DiscretePoint, x, y);