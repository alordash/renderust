use std::ops::{Add, Div, Mul, Range, Sub};

pub trait Interpolable: Copy + PartialOrd + Add<Output = Self> + Sub<Output = Self> {}

impl<T: Copy + PartialOrd + Add<Output = Self> + Sub<Output = Self>> Interpolable for T {}

pub struct Interpolator<T: Interpolable> {
    begin: T,
    end: T,
    t: T,
}

impl<T: Interpolable> Interpolator<T> {
    pub fn new(mut begin: T, mut end: T) -> Interpolator<T> {
        if end < begin {
            (begin, end) = (end, begin);
        }
        let t = end - begin;
        Interpolator { begin, end, t }
    }

    pub fn interpolate<U>(
        &self,
        phase: T,
        interpolation_value: U,
        begin_interpolation_value: U,
    ) -> U
    where
        U: Mul<T, Output = U> + Div<T, Output = U> + Add<U, Output = U>,
    {
        let dphase = phase - self.begin;
        (interpolation_value * dphase) / self.t + begin_interpolation_value
    }

    pub fn get_interpolation_range(&self) -> Range<T> {
        self.begin..self.end
    }

    pub fn get_begin(&self) -> &T {
        &self.begin
    }

    pub fn get_end(&self) -> &T {
        &self.end
    }

    pub fn get_diff(&self) -> &T {
        &self.t
    }
}

impl<T: Interpolable> From<(T, T)> for Interpolator<T> {
    fn from(source: (T, T)) -> Self {
        Interpolator::new(source.0, source.1)
    }
}
