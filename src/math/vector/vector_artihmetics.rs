use std::ops::{Add, Mul, Sub, Div};

use num::{Num, traits::AsPrimitive};

use super::math_vec::MathVec;

impl<const N: usize, T> Add<MathVec<N, T>> for MathVec<N, T>
where
    T: Num + Default + Copy,
{
    type Output = Self;
    default fn add(self, rhs: Self) -> Self::Output {
        let mut result = [T::default(); N];
        result
            .iter_mut()
            .zip(self.values().into_iter())
            .zip(rhs.values().into_iter())
            .for_each(|((rv, sv), rhsv)| *rv = *sv + *rhsv);
        MathVec::new(result)
    }
}

impl<const N: usize, T> Sub<MathVec<N, T>> for MathVec<N, T>
where
    T: Num + Default + Copy,
{
    type Output = Self;
    default fn sub(self, rhs: Self) -> Self::Output {
        let mut result = [T::default(); N];
        result
            .iter_mut()
            .zip(self.values().into_iter())
            .zip(rhs.values().into_iter())
            .for_each(|((rv, sv), rhsv)| *rv = *sv - *rhsv);
        MathVec::new(result)
    }
}

impl<const N: usize, T, U> Mul<U> for MathVec<N, T>
where
    T: Num + Copy + 'static,
    U: Num + AsPrimitive<T>
{
    type Output = Self;
    default fn mul(self, rhs: U) -> Self::Output {
        let mut result = self;
        for result_val in result.values_mut().iter_mut() {
            *result_val = *result_val * rhs.as_();
        }
        result
    }
}

impl<const N: usize, T, U> Div<U> for MathVec<N, T>
where
    T: Num + Copy + 'static,
    U: Num + AsPrimitive<T>
{
    type Output = Self;
    default fn div(self, rhs: U) -> Self::Output {
        let mut result = self;
        for result_val in result.values_mut().iter_mut() {
            *result_val = *result_val / rhs.as_();
        }
        result
    }
}
