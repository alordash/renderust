use super::discrete_point::DiscretePoint;

pub struct DiscretePolygon<const N: usize>(pub [DiscretePoint; N]);
