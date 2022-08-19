use std::str::FromStr;

use super::{Vec3, Vec3ParsingError};


pub type Vec3ui = Vec3<usize>;

impl FromStr for Vec3ui {
    type Err = Vec3ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut int_strings = s.split(' ');
        let mut vec3i = Vec3ui::default();
        for i in 0..3 {
            let int_string = int_strings.next().ok_or(Vec3ParsingError::NotEnoughItems)?;
            let single_int_string =
                int_string
                    .split('/')
                    .next()
                    .ok_or(Vec3ParsingError::FormatError(String::from(
                        "no backslash splitters",
                    )))?;
            let int = single_int_string
                .parse::<usize>()
                .map_err(|e| Vec3ParsingError::ParseError(Box::new(e)))?;
            unsafe {
                *vec3i.0.get_unchecked_mut(i) = int;
            }
        }

        Ok(vec3i)
    }
}