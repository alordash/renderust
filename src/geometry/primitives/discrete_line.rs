use super::discrete_point::DiscretePoint;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DiscreteLine {
    pub begin: DiscretePoint,
    pub end: DiscretePoint,
}
