use std::ops::Range;

use crate::geometry::primitives::{discrete_line::DiscreteLine, discrete_point::DiscretePoint};

pub struct DiscreteLineXAxisCalculator {
    begin: DiscretePoint,
    end: DiscretePoint,
    dx: isize,
    dy: isize,
}

impl DiscreteLineXAxisCalculator {
    pub fn new(mut begin: DiscretePoint, mut end: DiscretePoint) -> DiscreteLineXAxisCalculator {
        if end.x < begin.x {
            (begin, end) = (end, begin);
        }
        let dx = end.x - begin.x;
        let dy = end.y - begin.y;
        DiscreteLineXAxisCalculator { begin, end, dx, dy }
    }

    pub fn calculate_y_value(&self, x: isize) -> isize {
        return (x - self.begin.x) * self.dy / self.dx + self.begin.y;
    }

    pub fn get_x_calculation_range(&self) -> Range<isize> {
        self.begin.x..self.end.x
    }
}

impl From<DiscreteLine> for DiscreteLineXAxisCalculator {
    fn from(line: DiscreteLine) -> Self {
        DiscreteLineXAxisCalculator::new(line.begin, line.end)
    }
}
