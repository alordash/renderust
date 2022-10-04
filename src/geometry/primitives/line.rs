use super::point::Point;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Line {
    pub begin: Point,
    pub end: Point,
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

    pub fn new(begin_x: isize, begin_y: isize, end_x: isize, end_y: isize) -> Line {
        Line {
            begin: Point {
                x: begin_x,
                y: begin_y,
            },
            end: Point { x: end_x, y: end_y },
        }
    }
}
