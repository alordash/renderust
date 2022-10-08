use super::{line::Line, point::Point2D};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Polygon<const N: usize> {
    points: [Point2D; N],
}

impl<const N: usize> Polygon<N> {
    pub fn new(points: [Point2D; N]) -> Polygon<N> {
        Polygon { points }
    }

    pub fn get_points(&self) -> &[Point2D; N] {
        &self.points
    }

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

impl<const N: usize> TryFrom<Vec<Point2D>> for Polygon<N> {
    type Error = String;
    fn try_from(points_vec: Vec<Point2D>) -> Result<Self, Self::Error> {
        let points = points_vec
            .into_iter()
            .take(N)
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|e| format!("Error creating polygon[{}]: {:?}", N, e))?;
        Ok(Polygon::new(points))
    }
}
