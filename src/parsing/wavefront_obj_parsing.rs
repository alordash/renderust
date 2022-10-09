use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use image::DynamicImage;

use crate::{
    math::vector::{common_vectors::vec3f::Vec3f, linear_algebra::LinAlgOperations},
    plane_buffer::plane_buffer::PlaneBuffer,
    wavefront::wavefront_obj::WavefrontObj,
};

use super::{
    math_vec_parsing::str_parse_math_vec, wavefront_obj_faces_parsing::str_parse_wavefront_faces,
};

const LINE_ENDINGS: [&'static str; 2] = ["\r\n", "\n"];

fn normal_map_vecs_from_rgb(normal_map_img: DynamicImage) -> PlaneBuffer<Vec3f> {
    let normals: Vec<Vec3f> = normal_map_img
        .to_rgb()
        .iter()
        .map(|c| *c as f32)
        .collect::<Vec<f32>>()
        .chunks_exact(3)
        .map(|rgb| unsafe {
            Vec3f::new([
                *rgb.get_unchecked(0),
                *rgb.get_unchecked(1),
                *rgb.get_unchecked(2),
            ])
            .normalized()
        })
        .collect();

    PlaneBuffer::new(
        100,
        100,
        crate::plane_buffer::plane_buffer::PlaneBufferCreateOption::RawSource(normals),
    )
}

impl WavefrontObj {
    pub fn from_file(
        model_source: &File,
        texture_source: &File,
        normal_map_source: &File,
    ) -> Result<WavefrontObj, String> {
        let mut line = String::new();
        let texture_reader = BufReader::new(texture_source);
        let texture = image::load(texture_reader, image::ImageFormat::TGA)
            .unwrap()
            .flipv();
        let normal_map_reader = BufReader::new(normal_map_source);
        let normal_map_img = image::load(normal_map_reader, image::ImageFormat::TGA)
            .unwrap()
            .flipv();
        let normal_map = normal_map_vecs_from_rgb(normal_map_img);
        let mut wavefront_obj = WavefrontObj {
            vertices: Default::default(),
            vertex_textures: Default::default(),
            vertex_normals: Default::default(),
            faces: Default::default(),
            texture,
            normal_map,
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
                    let vec3f = str_parse_math_vec(&floats_string)?;
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
                        .for_each(|vec| vec.values_mut().iter_mut().for_each(|v| *v -= 1));
                    wavefront_obj.faces.push(vec3is);
                }
                _ => (),
            };

            line.clear();
        }

        Ok(wavefront_obj)
    }
}
