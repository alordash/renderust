#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DiscretePoint {
    pub x: isize,
    pub y: isize,
}

impl From<(isize, isize)> for DiscretePoint {
    fn from(source: (isize, isize)) -> Self {
        DiscretePoint {
            x: source.0,
            y: source.1,
        }
    }
}
