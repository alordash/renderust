#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RectSize {
    pub width: usize,
    pub height: usize,
}

impl From<(usize, usize)> for RectSize {
    fn from(source: (usize, usize)) -> Self {
        RectSize {
            width: source.0,
            height: source.1,
        }
    }
}
