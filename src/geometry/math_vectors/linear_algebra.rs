use std::ops::Mul;

use num::{Float, Num};

use super::Vec3;

impl<T> Vec3<T>
where
    T: Num + Copy,
{
    pub fn cross_product(self, rhs: Self) -> Self {
        Vec3::<T>([
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        ])
    }

    pub fn dot_product(self, rhs: Self) -> T {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }
}

impl<T> Vec3<T>
where
    T: Float,
{
    pub fn norm(&self) -> T {
        (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
    }

    pub fn normalize(&mut self) {
        let norm = self.norm();
        let multiplier = T::one() / norm;
        let result = *self * multiplier;
        *self = result;
    }
}
