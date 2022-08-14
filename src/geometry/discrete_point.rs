#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DiscretePoint {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for DiscretePoint {
    fn from(source: (usize, usize)) -> Self {
        DiscretePoint {
            x: source.0,
            y: source.1,
        }
    }
}
