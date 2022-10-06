use std::ops::Range;

use crate::{
    drawin::color::Color,
    geometry::primitives::{line::Line, point::Point},
};

pub struct LineXAxisCalculator {
    begin: Point,
    end: Point,
    dx: isize,
    dy: isize,
    dz: isize,
}

impl LineXAxisCalculator {
    pub fn new(mut begin: Point, mut end: Point) -> LineXAxisCalculator {
        if end.x() < begin.x() {
            (begin, end) = (end, begin);
        }
        let dx = end.x() - begin.x();
        let dy = end.y() - begin.y();
        let dz = end.z() - begin.z();
        LineXAxisCalculator {
            begin,
            end,
            dx,
            dy,
            dz,
        }
    }

    pub fn calculate_y_value(&self, x: isize) -> isize {
        // attempt to divide by zero
        (x - self.begin.x()) * self.dy / self.dx + self.begin.y()
    }

    pub fn calculate_y_and_z_value(&self, x: isize) -> (isize, isize) {
        // attempt to divide by zero
        let d = x - self.begin.x();
        (
            d * self.dy / self.dx + self.begin.y(),
            d * self.dz / self.dx + self.begin.z(),
        )
    }

    pub fn calculate_y_and_z_and_color_value(&self, x: isize) -> (isize, isize, Color) {
        // attempt to divide by zero
        let d = x - self.begin.x();
        let color = self
            .begin
            .color
            .interpolate(self.end.color, d as i32, self.dx as i32);
        (
            d * self.dy / self.dx + self.begin.y(),
            d * self.dz / self.dx + self.begin.z(),
            color,
        )
    }

    pub fn calculate_y_and_color_value(&self, x: isize) -> (isize, Color) {
        // attempt to divide by zero
        let d = x - self.begin.x();
        let color = self
            .begin
            .color
            .interpolate(self.end.color, d as i32, self.dx as i32);
        (d * self.dy / self.dx + self.begin.y(), color)
    }

    pub fn get_x_calculation_range(&self) -> Range<isize> {
        self.begin.x()..self.end.x()
    }
}

impl From<Line> for LineXAxisCalculator {
    fn from(line: Line) -> Self {
        LineXAxisCalculator::new(line.begin, line.end)
    }
}
