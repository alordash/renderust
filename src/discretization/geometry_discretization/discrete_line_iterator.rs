use crate::geometry::primitives::{discrete_line::DiscreteLine, discrete_point::DiscretePoint};

pub struct DiscreteLineIterator {
    pub begin: DiscretePoint,
    pub end: DiscretePoint,
    pub angle_bigger_45_deg: bool,
    pub dx: isize,
    pub dy_error: isize,
    pub y_error: isize,
    pub x: isize,
    pub y: isize,
}

impl DiscreteLineIterator {
    pub fn get_iterations_count(&self) -> usize {
        (self.end.x - self.begin.x) as usize
    }
}

impl Iterator for DiscreteLineIterator {
    type Item = DiscretePoint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.end.x {
            let result = if self.angle_bigger_45_deg {
                DiscretePoint {
                    x: self.y,
                    y: self.x,
                }
            } else {
                DiscretePoint {
                    x: self.x,
                    y: self.y,
                }
            };

            self.y_error += self.dy_error;
            if self.y_error > self.dx {
                // This condition can be optimized
                // by taking it out of loop
                if self.end.y > self.begin.y {
                    self.y += 1;
                } else {
                    self.y -= 1;
                }
                self.y_error -= self.dx * 2;
            }
            self.x += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl IntoIterator for DiscreteLine {
    type Item = DiscretePoint;
    type IntoIter = DiscreteLineIterator;
    fn into_iter(self) -> Self::IntoIter {
        let mut p1 = self.begin;
        let mut p2 = self.end;

        let mut dx = p2.x - p1.x;
        let mut dy = p2.y - p1.y;
        let mut angle_bigger_45_deg = false;

        if dy.abs() > dx.abs() {
            angle_bigger_45_deg = true;
            (p1.x, p1.y) = (p1.y, p1.x);
            (p2.x, p2.y) = (p2.y, p2.x);
        }

        if p2.x < p1.x {
            (p1, p2) = (p2, p1);
        }

        dx = p2.x - p1.x;
        dy = p2.y - p1.y;

        let dy_error = dy.abs() * 2;
        let y_error = 0;
        let y = p1.y;

        DiscreteLineIterator {
            angle_bigger_45_deg,
            begin: p1,
            end: p2,
            dx,
            dy_error,
            y_error,
            x: p1.x,
            y,
        }
    }
}
