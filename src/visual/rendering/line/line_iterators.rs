use glam::{IVec2, IVec3};

use crate::math::geometry::primitives::line::Line;

pub struct LineIterator {
    pub begin: IVec3,
    pub end: IVec3,
    pub angle_bigger_45_deg: bool,
    pub dx: i32,
    pub dy_error: i32,
    pub y_error: i32,
    pub x: i32,
    pub y: i32,
}

impl LineIterator {
    pub fn get_iterations_count(&self) -> usize {
        (self.end.x - self.begin.x) as usize
    }
}

impl Iterator for LineIterator {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.end.x {
            let result = if self.angle_bigger_45_deg {
                IVec2::new(self.y, self.x)
            } else {
                IVec2::new(self.x, self.y)
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

impl IntoIterator for Line {
    type Item = IVec2;
    type IntoIter = LineIterator;
    fn into_iter(self) -> Self::IntoIter {
        let mut p1 = self.begin;
        let mut p2 = self.end;

        let mut dx = (p2.x - p1.x) as i32;
        let mut dy = (p2.y - p1.y) as i32;
        let mut angle_bigger_45_deg = false;

        if dy.abs() > dx.abs() {
            angle_bigger_45_deg = true;
            (p1.x, p1.y) = (p1.y, p1.x);
            (p2.x, p2.y) = (p2.y, p2.x);
        }

        if p2.x < p1.x {
            (p1, p2) = (p2, p1);
        }

        dx = (p2.x - p1.x) as i32;
        dy = (p2.y - p1.y) as i32;

        let dy_error = dy.abs() as i32 * 2;
        let y_error = 0;
        let y = p1.y as i32;

        LineIterator {
            angle_bigger_45_deg,
            begin: IVec3::new(p1.x as i32, p1.y as i32, p1.z as i32),
            end: IVec3::new(p2.x as i32, p2.y as i32, p2.z as i32),
            dx,
            dy_error,
            y_error,
            x: p1.x as i32,
            y,
        }
    }
}
