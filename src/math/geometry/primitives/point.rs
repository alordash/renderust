use std::ops::{Deref, DerefMut};

use glam::{Vec2, Vec3A, IVec2};

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Point2D {
    pub coords: IVec2,
    z_depth: i32,
    uv: Vec2,
    normal: Vec3A,
}

impl Point2D {
    pub fn new(coords: [i32; 2]) -> Point2D {
        Point2D {
            coords: IVec2::from_array(coords),
            ..Default::default()
        }
    }

    pub fn new_full(coords: [i32; 2], z_depth: i32, uv: Vec2, normal: Vec3A) -> Point2D {
        Point2D {
            coords: IVec2::from_array(coords),
            z_depth,
            uv,
            normal,
        }
    }

    pub fn from(x: i32, y: i32) -> Point2D {
        Point2D::new([x, y])
    }

    pub fn get_z_depth(&self) -> &i32 {
        &self.z_depth
    }

    pub fn get_uv(&self) -> &Vec2 {
        &self.uv
    }

    pub fn get_normal(&self) -> &Vec3A {
        &self.normal
    }

    pub fn get_z_depth_mut(&mut self) -> &mut i32 {
        &mut self.z_depth
    }

    pub fn get_uv_mut(&mut self) -> &mut Vec2 {
        &mut self.uv
    }

    pub fn get_normal_mut(&mut self) -> &mut Vec3A {
        &mut self.normal
    }
}

impl From<(i32, i32)> for Point2D {
    fn from(source: (i32, i32)) -> Self {
        Point2D::new([source.0, source.1])
    }
}

impl Deref for Point2D {
    type Target = IVec2;
    fn deref(&self) -> &Self::Target {
        &self.coords
    }
}

impl DerefMut for Point2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.coords
    }
}
