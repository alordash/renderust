use std::ops::{Deref, DerefMut};

use num::{traits::AsPrimitive, Float, Num};

use crate::derive_xyzw;

use super::linear_algebra::LinAlgOperations;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MathVec<const N: usize, T>([T; N])
where
    T: Sized;

impl<const N: usize, T> MathVec<N, T> {
    pub fn new(values: [T; N]) -> Self {
        MathVec(values)
    }

    pub fn values(&self) -> &[T; N] {
        &self.0
    }

    pub fn values_mut(&mut self) -> &mut [T; N] {
        &mut self.0
    }

    pub fn consume_values(self) -> [T; N] {
        self.0
    }

    pub fn set_values(&mut self, rhs: &[T; N])
    where
        T: Copy,
    {
        self.0.copy_from_slice(rhs);
    }
}

impl<const N: usize, T: Num + Copy> LinAlgOperations<T> for MathVec<N, T> {
    default fn dot_product(self, rhs: Self) -> T {
        self.values()
            .iter()
            .zip(rhs.values().iter())
            .map(|(sv, rv)| *sv * *rv)
            .fold(T::zero(), |acc, x| acc + x)
    }
    default fn norm(&self) -> T
    where
        T: Float,
    {
        self.dot_product(*self).sqrt()
    }
    default fn normalize(&mut self)
    where
        T: Float + AsPrimitive<T>,
    {
        let norm = self.norm();
        let result = *self / norm;
        *self = result;
    }
    default fn normalized(&self) -> Self
    where
        T: Float + AsPrimitive<T>,
    {
        let mut self_clone = self.clone();
        self_clone.normalize();
        self_clone
    }
}

impl<const N: usize, T> Deref for MathVec<N, T> {
    type Target = [T; N];
    fn deref(&self) -> &Self::Target {
        self.values()
    }
}

impl<const N: usize, T> DerefMut for MathVec<N, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.values_mut()
    }
}

impl<const N: usize, T: Default + Copy> Default for MathVec<N, T> {
    fn default() -> Self {
        MathVec::new([T::default(); N])
    }
}

derive_xyzw!(MathVec<2, T>, T);
derive_xyzw!(MathVec<3, T>, T);
derive_xyzw!(MathVec<4, T>, T);
