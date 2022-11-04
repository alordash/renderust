use std::ops::{Deref, DerefMut};

use glam::{Vec2, Vec3A};

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Vertex {
    pub screen_pos: Vec3A,
    pub uv: Vec2,
    pub normal: Vec3A,
}

impl Vertex {
    pub fn new(screen_pos: Vec3A, uv: Vec2, normal: Vec3A) -> Vertex {
        Vertex { screen_pos, uv, normal }
    }
}

impl Deref for Vertex {
    type Target = Vec3A;
    fn deref(&self) -> &Self::Target {
        &self.screen_pos
    }
}

impl DerefMut for Vertex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.screen_pos
    }
}
