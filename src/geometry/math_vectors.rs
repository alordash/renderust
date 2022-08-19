pub mod vec3f;
pub mod vec3ui;

#[derive(Debug, thiserror::Error)]
pub enum Vec3ParsingError {
    #[error("not enough items")]
    NotEnoughItems,
    #[error("error parsing: `{0}`")]
    ParseError(Box<dyn std::error::Error>),
    #[error("format error: `{0}`")]
    FormatError(String),
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Vec3<T>(pub [T; 3])
where
    T: Sized;

impl<T> Vec3<T> {
    pub fn x(&self) -> &T {
        &self.0[0]
    }

    pub fn y(&self) -> &T {
        &self.0[1]
    }

    pub fn z(&self) -> &T {
        &self.0[2]
    }
}
