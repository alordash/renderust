use glam::{UVec3, Vec3A};
use image::DynamicImage;

use crate::plane_buffer::plane_buffer::PlaneBuffer;

#[derive(Clone, Debug)]
pub struct WavefrontObj {
    pub vertices: Vec<Vec3A>,
    pub vertex_textures: Vec<Vec3A>,
    pub vertex_normals: Vec<Vec3A>,
    pub faces: Vec<Vec<UVec3>>,
    pub texture: DynamicImage,
    pub normal_map: Option<PlaneBuffer<Vec3A>>,
    pub spec_map: Option<DynamicImage>,
    pub glow_map: Option<DynamicImage>
}
