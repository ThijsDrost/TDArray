use crate::array::{Array1D, Array2D, Array3D, Array4D, Array5D};
use std::iter::Iterator;

struct ArrayIterator<const D1: usize> {
    from_size: [usize; D1],
    to_size: [usize; D1],
    index: [usize; D1],
}

impl<const D1: usize> ArrayIterator<{D1}>
{
    fn new(from_size: [usize; D1], to_size: [usize; D1]) -> Self {
        let mut index = [0; D1];
        let mut t_size = [0; D1];
        for i in 0..D1 {
            t_size[i] = to_size[i];
        }
        let mut f_size = [0; D1];
        for i in 0..D1 {
            f_size[i] = from_size[i];
        }
        ArrayIterator {
            from_size,
            to_size,
            index,
        }
    }
}

impl<const D1: usize> Iterator for ArrayIterator<D1> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.index[0] += 1;
        for i in 0..D1 {
            if self.index[i] == self.to_size[i] {
                if i + 1 == D1 {
                    return None;
                }
                self.index[i] = 0;
                self.index[i + 1] += 1;
            }
        }
        let mut total = 0;
        for i in 0..D1 {
            total += self.index[i] * (self.from_size[i] - 1);
        }
        Some(total)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut total = 1;
        let mut been = 0;
        for i in 0..D1 {
            been += self.index[i]*total;
            total *= self.to_size[i];
        }
        total -= been;
        (total, Some(total))
    }
}

