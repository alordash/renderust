use super::discrete_point::DiscretePoint;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DiscretePolygon<const N: usize> {
    pub points: [DiscretePoint; N],
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
