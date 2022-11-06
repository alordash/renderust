use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use glam::{UVec3, Vec3A};
use image::{DynamicImage, GenericImageView};

use crate::{
    plane_buffer::plane_buffer::PlaneBuffer,
    wavefront::{wavefront_obj::WavefrontObj, wavefront_obj_source::WaveFrontObjSource},
};

use super::{
    math_vec_parsing::str_parse_vec3, wavefront_obj_faces_parsing::str_parse_wavefront_faces,
};

const LINE_ENDINGS: [&'static str; 2] = ["\r\n", "\n"];

fn normal_map_vecs_from_rgb(normal_map_img: DynamicImage) -> PlaneBuffer<Vec3A> {
    let normals: Vec<Vec3A> = normal_map_img
        .to_rgb8()
        .iter()
        .map(|c| (*c as f32 / 255.0) * 2.0 - 1.0)
        .collect::<Vec<f32>>()
        .chunks_exact(3)
        .map(Vec3A::from_slice)
        .map(Vec3A::normalize)
        .collect();

    PlaneBuffer::new(
        normal_map_img.width() as usize,
        normal_map_img.height() as usize,
        crate::plane_buffer::plane_buffer::PlaneBufferCreateOption::RawSource(normals),
    )
}

impl WavefrontObj {
    pub fn from_file(
        model_source: &File,
        texture_source: &File,
        normal_map_source: Option<&File>,
        spec_map_source: Option<&File>,
        glow_map_source: Option<&File>,
    ) -> Result<WavefrontObj, String> {
        let mut line = String::new();
        let texture_reader = BufReader::new(texture_source);
        let texture = image::load(texture_reader, image::ImageFormat::Tga)
            .unwrap()
            .flipv();

        let normal_map = normal_map_source
            .map(BufReader::new)
            .map(|reader| image::load(reader, image::ImageFormat::Tga).unwrap())
            .map(normal_map_vecs_from_rgb);

        let spec_map = spec_map_source.map(BufReader::new).map(|reader| {
            image::load(reader, image::ImageFormat::Tga)
                .unwrap()
                .flipv()
        });

        let glow_map = glow_map_source.map(BufReader::new).map(|reader| {
            image::load(reader, image::ImageFormat::Tga)
                .unwrap()
                .flipv()
        });

        let mut wavefront_obj = WavefrontObj {
            vertices: Default::default(),
            vertex_textures: Default::default(),
            vertex_normals: Default::default(),
            faces: Default::default(),
            texture,
            normal_map,
            spec_map,
            glow_map,
        };
        let mut buff_reader = BufReader::new(model_source);

        loop {
            match buff_reader.read_line(&mut line) {
                Ok(0) => break,
                Err(_) => break,
                _ => (),
            };

            for line_ending in LINE_ENDINGS.iter() {
                if line.ends_with(line_ending) {
                    line.truncate(line.len() - line_ending.len());
                    break;
                }
            }

            let first_letters: String = line.chars().take_while(|c| *c != ' ').collect();

            match first_letters.as_str() {
                "v" | "vt" | "vn" => {
                    let floats_string: String =
                        line.chars().skip(first_letters.len() + 1).collect();
                    let vec3f = str_parse_vec3(&floats_string)?;
                    match first_letters.as_str() {
                        "v" => wavefront_obj.vertices.push(vec3f),
                        "vt" => wavefront_obj.vertex_textures.push(vec3f),
                        "vn" => wavefront_obj.vertex_normals.push(vec3f),
                        _ => (),
                    };
                }
                "f" => {
                    let ints_string: String = line.chars().skip(first_letters.len() + 1).collect();

                    let mut vec3is = str_parse_wavefront_faces(&ints_string)?;
                    vec3is
                        .iter_mut()
                        .for_each(|vec| *vec -= UVec3::new(1, 1, 1));
                    wavefront_obj.faces.push(vec3is);
                }
                _ => (),
            };

            line.clear();
        }

        Ok(wavefront_obj)
    }

    pub fn from_paths(
        model_source_path: &Path,
        texture_source_path: &Path,
        normal_map_source_path: Option<&Path>,
        spec_map_source_path: Option<&Path>,
        glow_map_source_path: Option<&Path>,
    ) -> Result<WavefrontObj, String> {
        let wavefront_obj_file = File::open(model_source_path)
            .map_err(|e| format!("Error opening model file: {:?}", e))?;
        let texture_file = File::open(texture_source_path)
            .map_err(|e| format!("Error opening texture file: {:?}", e))?;

        let normal_map_file = normal_map_source_path
            .map(File::open)
            .map(|f| f.map_err(|e| format!("Error opening normal map file: {:?}", e)))
            .map(Result::unwrap);

        let spec_map_file = spec_map_source_path
            .map(File::open)
            .map(|f| f.map_err(|e| format!("Error opening spec map file: {:?}", e)))
            .map(Result::unwrap);

        let glow_map_file = glow_map_source_path
            .map(File::open)
            .map(|f| f.map_err(|e| format!("Error opening glow map file: {:?}", e)))
            .map(Result::unwrap);

        WavefrontObj::from_file(
            &wavefront_obj_file,
            &texture_file,
            normal_map_file.as_ref(),
            spec_map_file.as_ref(),
            glow_map_file.as_ref(),
        )
        .map_err(|e| format!("Error parsing file: {:?}", e))
    }

    pub fn from_sources_struct(
        wavefront_obj_source: &WaveFrontObjSource,
    ) -> Result<WavefrontObj, String> {
        WavefrontObj::from_paths(
            wavefront_obj_source.model_path.as_ref(),
            wavefront_obj_source.texture_path.as_ref(),
            wavefront_obj_source.normal_map_path.map(|s| s.as_ref()),
            wavefront_obj_source.spec_map_path.map(|s| s.as_ref()),
            wavefront_obj_source.glow_map_path.map(|s| s.as_ref()),
        )
    }
}
