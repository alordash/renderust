use image::DynamicImage;

use crate::{math::vector::common_vectors::{vec3f::Vec3f, vec3ui::Vec3ui}, plane_buffer::plane_buffer::PlaneBuffer};

pub struct WavefrontObj {
    pub vertices: Vec<Vec3f>,
    pub vertex_textures: Vec<Vec3f>,
    pub vertex_normals: Vec<Vec3f>,
    pub faces: Vec<Vec<Vec3ui>>,
    pub texture: DynamicImage,
    pub normal_map: PlaneBuffer<Vec3f>
}
