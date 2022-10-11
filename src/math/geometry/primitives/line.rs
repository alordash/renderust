use super::point::Point2D;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Line {
    pub begin: Point2D,
    pub end: Point2D,
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

    pub fn new(begin_x: i32, begin_y: i32, end_x: i32, end_y: i32) -> Line {
        Line {
            begin: Point2D::from(begin_x, begin_y),
            end: Point2D::from(end_x, end_y)
        }
    }
}
