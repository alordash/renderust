use num::Num;

use crate::math::vector::{linear_algebra::LinAlgOperations, math_vec::MathVec};

pub type Vec3<T> = MathVec<3, T>;

impl<T: Num + Copy> Vec3<T> {
    pub fn cross_product(self, rhs: Self) -> Self {
        MathVec::new([
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        ])
    }
}

impl<T> LinAlgOperations<T> for Vec3<T>
where
    T: Num + Copy,
{
    fn dot_product(self, rhs: Self) -> T {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }
}
