use super::discrete_point::DiscretePoint;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DiscreteLine {
    pub begin: DiscretePoint,
    pub end: DiscretePoint,
}

impl DiscreteLine {
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

    pub fn new(begin_x: isize, begin_y: isize, end_x: isize, end_y: isize) -> DiscreteLine {
        DiscreteLine {
            begin: DiscretePoint {
                x: begin_x,
                y: begin_y,
            },
            end: DiscretePoint { x: end_x, y: end_y },
        }
    }
}
