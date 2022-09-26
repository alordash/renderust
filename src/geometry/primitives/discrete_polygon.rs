use super::{discrete_line::DiscreteLine, discrete_point::DiscretePoint};oint::DiscretePoint;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DiscretePolygon<const N: usize> {
    pub points: [DiscretePoint; N],
}

impl<const N: usize> DiscretePolygon<N> {
    pub fn get_perimeter_lines(&self) -> Vec<DiscreteLine> {
        let mut lines: Vec<_> = self
            .points
            .windows(2)
            .map(|two_points| unsafe {
                DiscreteLine {
                    begin: *two_points.get_unchecked(0),
                    end: *two_points.get_unchecked(1),
                }
            })
            .collect();
        lines.push(DiscreteLine {
            begin: self.points[self.points.len() - 1],
            end: self.points[0],
        });
        lines
    }
}

impl<const N: usize> From<Vec<DiscretePoint>> for DiscretePolygon<N> {
    fn from(points_vec: Vec<DiscretePoint>) -> Self {
        DiscretePolygon {
            points: points_vec
                .into_iter()
                .take(N)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}
