use crate::array::Array1D;
use num_traits::{Pow};

impl<T: Pow<T, Output=T>, const D1: usize> Array1D<T, D1> {
    pub fn pow(&self, exp: T) -> Array1D<T, D1> {
        let data = core::array::from_fn(|i| self.data[i].pow(exp));
        Array1D{data}
    }
}