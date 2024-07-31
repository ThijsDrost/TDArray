use crate::array::{Array1D, Array2D, Array3D, Array4D, Array5D};
use num_traits::{Pow};
use std::ops::{Mul, Add, Div, Sub, AddAssign, SubAssign, MulAssign, DivAssign};
use proc_macro::{mk_impl_math, mk_impl_math_inplace, mk_impl_reshape, mk_over_nums};

mk_over_nums!(mk_impl_math!(Add, add, rhs, $1); #5);
mk_over_nums!(mk_impl_math!(Div, div, rhs, $1); #5);
mk_over_nums!(mk_impl_math!(Mul, mul, rhs, $1); #5);
mk_over_nums!(mk_impl_math!(Sub, sub, rhs, $1); #5);
mk_over_nums!(mk_impl_math!(Pow, pow, exponent, $1); #5);

mk_over_nums!(mk_impl_math_inplace!(AddAssign, add_assign, rhs, $1); #5);
mk_over_nums!(mk_impl_math_inplace!(SubAssign, sub_assign, rhs, $1); #5);
mk_over_nums!(mk_impl_math_inplace!(MulAssign, mul_assign, rhs, $1); #5);
mk_over_nums!(mk_impl_math_inplace!(DivAssign, div_assign, rhs, $1); #5);

mk_over_nums!(mk_impl_reshape!($1 to $2); #5 #5);
