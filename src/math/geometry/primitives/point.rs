use std::ops::{Deref, DerefMut};

use crate::{
    math::vector::common_vectors::{vec2::Vec2, vec2f::Vec2f, vec3f::Vec3f},
    visual::color::color::Color,
};

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub struct Point2D {
    coords: Vec2<isize>,
    z_depth: isize,
    uv: Vec2f,
    normal: Vec3f,
    color: Option<Color>,
}

impl Point2D {
    pub fn new(coords: [isize; 2]) -> Point2D {
        Point2D {
            coords: Vec2::new(coords),
            ..Default::default()
        }
    }

    pub fn new_full(coords: [isize; 2], z_depth: isize, uv: Vec2f, normal: Vec3f) -> Point2D {
        Point2D {
            coords: Vec2::new(coords),
            z_depth,
            uv,
            normal,
            color: None,
        }
    }

    pub fn from(x: isize, y: isize) -> Point2D {
        Point2D::new([x, y])
    }

    pub fn get_z_depth(&self) -> &isize {
        &self.z_depth
    }

    pub fn get_uv(&self) -> &Vec2f {
        &self.uv
    }

    pub fn get_normal(&self) -> &Vec3f {
        &self.normal
    }

    pub fn get_color(&self) -> &Option<Color> {
        &self.color
    }

    pub fn get_z_depth_mut(&mut self) -> &mut isize {
        &mut self.z_depth
    }

    pub fn get_uv_mut(&mut self) -> &mut Vec2f {
        &mut self.uv
    }

    pub fn get_normal_mut(&mut self) -> &mut Vec3f {
        &mut self.normal
    }

    pub fn get_color_mut(&mut self) -> &mut Option<Color> {
        &mut self.color
    }
}

impl From<(isize, isize)> for Point2D {
    fn from(source: (isize, isize)) -> Self {
        Point2D::new([source.0, source.1])
    }
}

impl Deref for Point2D {
    type Target = Vec2<isize>;
    fn deref(&self) -> &Self::Target {
        &self.coords
    }
}

impl DerefMut for Point2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.coords
    }
}
