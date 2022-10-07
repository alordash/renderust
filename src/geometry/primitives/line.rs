use crate::{discretization::geometry_discretization::line_x_axis_calculator::LineXAxisCalculator, drawin::draw_buffer::DrawBuffer};

use super::point::Point;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Line {
    pub begin: Point,
    pub end: Point,
}

impl Line {
    pub fn order_by_x(&mut self) {
        if self.begin.x() > self.end.x() {
            (self.begin, self.end) = (self.end, self.begin);
        }
    }

    pub fn order_by_y(&mut self) {
        if self.begin.y() > self.end.y() {
            (self.begin, self.end) = (self.end, self.begin);
        }
    }

    pub fn new(begin_x: isize, begin_y: isize, end_x: isize, end_y: isize) -> Line {
        Line {
            begin: Point::new(begin_x, begin_y),
            end: Point::new(end_x, end_y)
        }
    }

    pub fn rough_draw(&self, canvas: &mut DrawBuffer) {
        let calculator = LineXAxisCalculator::new(self.begin, self.end);
        for x in self.begin.x()..self.end.x() {
            let (y, color) = calculator.calculate_y_and_color_value(x);
            canvas[(x as usize, y as usize)] = color;
        }
    }
}
