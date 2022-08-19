use std::str::FromStr;

use super::{Vec3, Vec3ParsingError};

pub type Vec3f = Vec3<f32>;

impl FromStr for Vec3f {
    type Err = Vec3ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut float_strings = s.split_whitespace();
        let mut vec3f = Vec3f::default();
        for i in 0..3 {
            let float_string = float_strings
                .next()
                .ok_or(Vec3ParsingError::NotEnoughItems)?;
            let float = float_string
                .parse::<f32>()
                .map_err(|e| Vec3ParsingError::ParseError(Box::new(e)))?;
            vec3f.0[i] = float;
        }

        Ok(vec3f)
    }
}