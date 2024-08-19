use crate::Array1D;
use std::iter::Iterator;

struct Array1DIterator<'a, T, const D1: usize> {
    data: &'a Array1D<T, D1>,
    index: usize,
    array_index: usize,
    every: usize,
}

impl<'a, T, const D1: usize> Iterator for Array1DIterator<'a, T, D1> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.array_index == self.data.data.len() {
            return None;
        }
        let val = self.array_index.clone();
        if self.index % self.every == 0 {
            self.array_index += 1;
        }
        self.index += 1;
        return self.data.data[val]
    }
}

