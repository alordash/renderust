use super::{Vec3, Vec3ParsingError};

pub type Vec3ui = Vec3<usize>;

impl Vec3ui {
    pub fn from_wavefront_str(s: &str) -> Result<Vec<Self>, Vec3ParsingError> {
        let mut vec3is = Vec::new();
        for i in 0..3 {
            let mut vec3i = Vec3ui::default();
            let mut int_strings = s.split(' ');
            for j in 0..3 {
                let int_string = int_strings.next().ok_or(Vec3ParsingError::NotEnoughItems)?;
                let single_int_string =
                    int_string
                        .split('/')
                        .skip(i)
                        .next()
                        .ok_or(Vec3ParsingError::FormatError(String::from(
                            "no backslash splitters",
                        )))?;
                let int = single_int_string
                    .parse::<usize>()
                    .map_err(|e| Vec3ParsingError::ParseError(Box::new(e)))?;
                unsafe {
                    *vec3i.0.get_unchecked_mut(j) = int;
                }
            }
            vec3is.push(vec3i)
        }

        Ok(vec3is)
    }
}
