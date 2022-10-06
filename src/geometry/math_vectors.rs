use std::str::FromStr;

use num::Num;

#[derive(Debug, thiserror::Error)]
pub enum Vec3ParsingError {
    #[error("not enough items")]
    NotEnoughItems,
    #[error("error parsing: `{0}`")]
    ParseError(Box<dyn std::error::Error>),
    #[error("format error: `{0}`")]
    FormatError(String),
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec3<T>(pub [T; 3])
where
    T: Sized;

impl<T> Vec3<T>
where
    T: Num + Copy,
{
    pub fn x(&self) -> T {
        self.0[0]
    }

    pub fn y(&self) -> T {
        self.0[1]
    }

    pub fn z(&self) -> T {
        self.0[2]
    }
}

impl<T: Default + FromStr> FromStr for Vec3<T>
where
    <T as FromStr>::Err: 'static + std::error::Error,
{
    type Err = Vec3ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut float_strings = s.split_whitespace();
        let mut vec3 = Vec3::default();
        for i in 0..3 {
            let value_string = float_strings
                .next()
                .ok_or(Vec3ParsingError::NotEnoughItems)?;
            let value = value_string
                .parse::<T>()
                .map_err(|e| Vec3ParsingError::ParseError(Box::new(e)))?;
            vec3.0[i] = value;
        }

        Ok(vec3)
    }
}

pub mod arithmetics;
pub mod linear_algebra;
pub mod vec3f;
pub mod vec3ui;
