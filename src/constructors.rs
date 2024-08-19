use crate::array::{Array1D, Array2D, Array3D, Array4D, Array5D};
use num_traits::{Float, Num, Zero, One};
use proc_macro::{mk_over_nums, mk_impl_from_fn};

impl <T: Float, const D1: usize> Array1D<T, D1> {
    pub fn linspace(start: T, stop: T) -> Self {
        let num = T::from(D1).unwrap();
        let data = core::array::from_fn(|i| start + (stop - start) * T::from(i).unwrap() / num);
        Array1D{data}
    }

    pub fn logspace(start: T, stop: T) -> Self {
        let num = T::from(D1).unwrap();
        let data = core::array::from_fn(|i| start + (stop - start) * T::from(i).unwrap() / num);
        Array1D{data}
    }
}


macro_rules! mk_impl_constructor {
    ($val: literal) => {
        mk_impl_from_fn!(Zero; T::zero(); zeros; $val;);
        mk_impl_from_fn!(One; T::one(); ones; $val;);
        mk_impl_from_fn!(Clone; value.clone(); full; $val; value: T);
    }
}

mk_over_nums!(mk_impl_constructor!($1); #5);