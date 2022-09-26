use std::ops::{Add, Mul, Sub, MulAssign};

use num::Num;

use super::Vec3;

impl<T> Add for Vec3<T>
where
    T: Num + Default,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::default();
        for ((result_val, self_val), rhs_val) in result
            .0
            .iter_mut()
            .zip(self.0.into_iter())
            .zip(rhs.0.into_iter())
        {
            *result_val = self_val + rhs_val;
        }
        result
    }
}

impl<T> Sub for Vec3<T>
where
    T: Num + Default,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Self::default();
        for ((result_val, self_val), rhs_val) in result
            .0
            .iter_mut()
            .zip(self.0.into_iter())
            .zip(rhs.0.into_iter())
        {
            *result_val = self_val - rhs_val;
        }
        result
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Num + Copy,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let mut result = self;
        for result_val in result.0.iter_mut() {
            *result_val = *result_val * rhs;
        }
        result
    }
}
