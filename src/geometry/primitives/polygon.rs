use super::{line::Line, point::Point};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Polygon<const N: usize> {
    pub points: [Point; N],
}

impl<const N: usize> Polygon<N> {
    pub fn get_perimeter_lines(&self) -> Vec<Line> {
        let mut lines: Vec<_> = self
            .points
            .windows(2)
            .map(|two_points| unsafe {
                Line {
                    begin: *two_points.get_unchecked(0),
                    end: *two_points.get_unchecked(1),
                }
            })
            .collect();
        lines.push(Line {
            begin: self.points[self.points.len() - 1],
            end: self.points[0],
        });
        lines
    }
}

impl<const N: usize> From<Vec<Point>> for Polygon<N> {
    fn from(points_vec: Vec<Point>) -> Self {
        Polygon {
            points: points_vec
                .into_iter()
                .take(N)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}
