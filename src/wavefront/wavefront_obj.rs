use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use image::DynamicImage;

use crate::geometry::math_vectors::{vec3f::Vec3f, vec3ui::Vec3ui, Vec3, Vec3ParsingError};

// #[derive(Debug, Default)]
pub struct WavefrontObj {
    pub vertices: Vec<Vec3f>,
    pub vertex_textures: Vec<Vec3f>,
    pub vertex_normals: Vec<Vec3f>,
    pub faces: Vec<Vec<Vec3ui>>,
    pub texture: DynamicImage,
}

const LINE_ENDINGS: [&'static str; 2] = ["\r\n", "\n"];

impl WavefrontObj {
    pub fn from_file(
        model_source: &File,
        texture_source: &File,
    ) -> Result<WavefrontObj, Vec3ParsingError> {
        let mut buff_reader = BufReader::new(model_source);

        let mut line = String::new();
        let mut texture_reader = BufReader::new(texture_source);
        let img = image::load(texture_reader, image::ImageFormat::TGA).unwrap().flipv();
        let mut wavefront_obj = WavefrontObj {
            vertices: Default::default(),
            vertex_textures: Default::default(),
            vertex_normals: Default::default(),
            faces: Default::default(),
            texture: img,
        };

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
                    let vec3f = floats_string.parse::<Vec3f>()?;
                    match first_letters.as_str() {
                        "v" => wavefront_obj.vertices.push(vec3f),
                        "vt" => wavefront_obj.vertex_textures.push(vec3f),
                        "vn" => wavefront_obj.vertex_normals.push(vec3f),
                        _ => (),
                    };
                }
                "f" => {
                    let ints_string: String = line.chars().skip(first_letters.len() + 1).collect();
                    let mut vec3is = Vec3ui::from_wavefront_str(&ints_string)?;
                    vec3is.iter_mut().for_each(|vec| vec.0.iter_mut().for_each(|v| * v -= 1));
                    wavefront_obj.faces.push(vec3is);
                }
                _ => (),
            };

            line.clear();
        }

        Ok(wavefront_obj)
    }
}
