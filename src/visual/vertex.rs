use std::ops::{Deref, DerefMut};

use glam::{Vec2, Vec3A};

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Vertex {
    pub pos: Vec3A,
    pub uv: Vec2,
    pub normal: Vec3A,
}

impl Vertex {
    pub fn new(pos: Vec3A, uv: Vec2, normal: Vec3A) -> Vertex {
        Vertex { pos, uv, normal }
    }
}

impl Deref for Vertex {
    type Target = Vec3A;
    fn deref(&self) -> &Self::Target {
        &self.pos
    }
}

impl DerefMut for Vertex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pos
    }
}
