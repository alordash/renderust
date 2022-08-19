use std::{
    num::{ParseFloatError, ParseIntError},
    str::FromStr,
};

#[derive(Debug)]
pub enum Vec3xParsingError {
    NotEnoughFloats,
    ParseFloatError(ParseFloatError),
    NotEnoughInts,
    NoBackslashSplitters,
    ParseIntError(ParseIntError),
}

#[derive(Default, Debug)]
pub struct Vec3f([f32; 3]);

impl FromStr for Vec3f {
    type Err = Vec3xParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut float_strings = s.split_whitespace();
        let mut vec3f = Vec3f::default();
        for i in 0..3 {
            let float_string = float_strings
                .next()
                .ok_or(Vec3xParsingError::NotEnoughFloats)?;
            let float = match float_string
                .parse::<f32>()
                .map_err(|e| Vec3xParsingError::ParseFloatError(e))
            {
                Ok(v) => v,
                Err(e) => {
                    println!("wtf: {:?}", &e);
                    return Err(e);
                }
            };
            vec3f.0[i] = float;
        }

        Ok(vec3f)
    }
}

#[derive(Default, Debug)]
pub struct Vec3i([i32; 3]);

impl FromStr for Vec3i {
    type Err = Vec3xParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut int_strings = s.split(' ');
        let mut vec3i = Vec3i::default();
        for i in 0..3 {
            let int_string = int_strings.next().ok_or(Vec3xParsingError::NotEnoughInts)?;
            let single_int_string = int_string
                .split('/')
                .next()
                .ok_or(Vec3xParsingError::NoBackslashSplitters)?;
            let int = single_int_string
                .parse::<i32>()
                .map_err(|e| Vec3xParsingError::ParseIntError(e))?;
            unsafe {
                *vec3i.0.get_unchecked_mut(i) = int;
            }
        }

        Ok(vec3i)
    }
}
