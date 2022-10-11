use glam::{UVec3, Vec3};
use image::DynamicImage;

use crate::plane_buffer::plane_buffer::PlaneBuffer;

pub struct WavefrontObj {
    pub vertices: Vec<Vec3>,
    pub vertex_textures: Vec<Vec3>,
    pub vertex_normals: Vec<Vec3>,
    pub faces: Vec<Vec<UVec3>>,
    pub texture: DynamicImage,
    pub normal_map: PlaneBuffer<Vec3>,
}
