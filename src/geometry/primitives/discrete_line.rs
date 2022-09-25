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
}
