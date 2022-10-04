use crate::{derive_self_add, derive_self_sub};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
}

impl From<(isize, isize)> for Point {
    fn from(source: (isize, isize)) -> Self {
        Point {
            x: source.0,
            y: source.1,
        }
    }
}

derive_self_add!(Point, x, y);
derive_self_sub!(Point, x, y);