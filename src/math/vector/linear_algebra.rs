use num::{Float, Num, traits::AsPrimitive};
pub trait LinAlgOperations<T: Num> {
    fn dot_product(self, rhs: Self) -> T;
    fn norm(&self) -> T
    where
        T: Float;
    fn normalize(&mut self)
    where
        T: Float + AsPrimitive<T>;
    fn normalized(&self) -> Self
    where
        T: Float + AsPrimitive<T>;
}
