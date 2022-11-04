use glam::Vec3A;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Line {
    pub begin: Vec3A,
    pub end: Vec3A,
}

impl Line {
    pub fn order_by_x(&mut self) {
        if self.begin.x > self.end.x {
            (self.begin, self.end) = (self.end, self.begin);
        }
    }

    pub fn order_by_y(&mut self) {
        if self.begin.y > self.end.y {
            (self.begin, self.end) = (self.end, self.begin);
        }
    }

    pub fn new(begin: Vec3A, end: Vec3A) -> Line {
        Line { begin, end }
    }
}
