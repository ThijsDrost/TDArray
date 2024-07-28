use crate::array::Array1D;
use num_traits::{Pow};
use std::ops::{Mul, Add, Div, Sub, AddAssign, SubAssign, MulAssign, DivAssign};

macro_rules! impl_math_ops {
    ($trait:ident, $method:ident) => {
        impl<T: Clone + $trait<T, Output=T>, const D1: usize> $trait<T> for Array1D<T, D1> where [(); D1]:,
        {
            type Output = Array1D<T, D1>;

            fn $method(self, rhs: T) -> Self::Output {
                let data = core::array::from_fn(|i| self.data[i].clone().$method(rhs.clone()));
                Array1D{data}
            }
        }
    };
}

impl_math_ops!(Add, add);
impl_math_ops!(Div, div);
impl_math_ops!(Mul, mul);
impl_math_ops!(Sub, sub);
impl_math_ops!(Pow, pow);

macro_rules! impl_math_ops_inplace {
    ($trait:ident, $method:ident) => {
        impl<T: Clone + $trait<T>, const D1: usize> $trait<T> for Array1D<T, D1> where [(); D1]:,
        {
            fn $method(&mut self, rhs: T) {
                for i in 0..D1 {
                    self.data[i].$method(rhs.clone());
                }
            }
        }
    };
}

impl_math_ops_inplace!(AddAssign, add_assign);
impl_math_ops_inplace!(SubAssign, sub_assign);
impl_math_ops_inplace!(MulAssign, mul_assign);
impl_math_ops_inplace!(DivAssign, div_assign);
