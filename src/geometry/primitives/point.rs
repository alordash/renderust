use crate::{derive_self_add, derive_self_sub, drawin::color::Color};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize,
    pub color: Color,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Point {
        Point {
            x,
            y,
            color: Color::default(),
        }
    }
}

impl From<(isize, isize)> for Point {
    fn from(source: (isize, isize)) -> Self {
        Point {
            x: source.0,
            y: source.1,
            color: Color::default(),
        }
    }
}

derive_self_add!(Point, x, y);
derive_self_sub!(Point, x, y);
