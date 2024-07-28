use crate::array::{Array1D, Array2D, Array3D, Array4D, Array5D};
use num_traits::{Float, Num, Zero, One};

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

impl <T: Zero, const D1: usize> Array1D<T, D1> {
    pub fn zeros() -> Self {
        let data = core::array::from_fn(|_| T::zero());
        Array1D{data}
    }
}

impl <T: One, const D1: usize> Array1D<T, D1> {
    pub fn ones() -> Self {
        let data = core::array::from_fn(|_| T::one());
        Array1D{data}
    }
}