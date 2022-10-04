use std::ops::Range;

use crate::geometry::primitives::{line::Line, point::Point};

pub struct LineXAxisCalculator {
    begin: Point,
    end: Point,
    dx: isize,
    dy: isize,
}

impl LineXAxisCalculator {
    pub fn new(mut begin: Point, mut end: Point) -> LineXAxisCalculator {
        if end.x < begin.x {
            (begin, end) = (end, begin);
        }
        let dx = end.x - begin.x;
        let dy = end.y - begin.y;
        LineXAxisCalculator { begin, end, dx, dy }
    }

    pub fn calculate_y_value(&self, x: isize) -> isize {
        // attempt to divide by zero
        (x - self.begin.x) * self.dy / self.dx + self.begin.y
    }

    pub fn get_x_calculation_range(&self) -> Range<isize> {
        self.begin.x..self.end.x
    }
}

impl From<Line> for LineXAxisCalculator {
    fn from(line: Line) -> Self {
        LineXAxisCalculator::new(line.begin, line.end)
    }
}
