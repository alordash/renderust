use num::Num;

use crate::math::vector::{linear_algebra::LinAlgOperations, math_vec::MathVec};

pub type Vec2<T> = MathVec<2, T>;

impl<T> LinAlgOperations<T> for Vec2<T>
where
    T: Num + Copy,
{
    fn dot_product(self, rhs: Self) -> T {
        self.x() * rhs.x() + self.y() * rhs.y()
    }
}
