use crate::{
    derive_self_add, derive_self_sub, derive_self_xyz, drawin::color::Color,
    geometry::math_vectors::Vec3,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub coords: Vec3<isize>,
    pub color: Color,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Point {
        Point {
            coords: Vec3([x, y, Default::default()]),
            color: Color::default(),
        }
    }

    pub fn new_with_z(x: isize, y: isize, z: isize) -> Point {
        Point {
            coords: Vec3([x, y, z]),
            color: Color::default(),
        }
    }

    pub fn new_with_color(x: isize, y: isize, color: Color) -> Point {
        Point {
            coords: Vec3([x, y, Default::default()]),
            color,
        }
    }
}

impl From<(isize, isize)> for Point {
    fn from(source: (isize, isize)) -> Self {
        Point {
            coords: Vec3([source.0, source.1, Default::default()]),
            color: Color::default(),
        }
    }
}

derive_self_xyz!(Point, coords, isize);

// derive_self_add!(Point, coords);
// derive_self_sub!(Point, coords);
