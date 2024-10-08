use std::ops::{Index, IndexMut, Add, Sub};
use std::clone::Clone;
use const_panic::concat_assert;
use proc_macro::{mk_over_nums, mk_array, mk_impl_clone, mk_impl_index, mk_impl_index_mut};

trait Array<T, U: Into<usize>>: Index<U> + IndexMut<U>
{
    type ArrayOutput;
    const NDIM: usize;
    const SIZE: usize;

    fn shape(&self) -> [usize; Self::NDIM];
    fn size(&self) -> usize {
        Self::SIZE
    }
    fn ndim(&self) -> usize {
        Self::NDIM
    }
    fn flatten(&self) -> Array1D<T, { Self::SIZE }>;

    fn unwrap_indexes(&self, indexes: [usize; Self::NDIM]) -> usize {
        let mut index = 0;
        let shape = self.shape();
        for i in 0..Self::NDIM {
            index += indexes[i]*shape[i];
        }
        index
    }

    fn wrap_index(&self, index: usize) -> [usize; Self::NDIM] {
        let mut indexes = [0; Self::NDIM];
        let shape = self.shape();
        let mut index = index;
        for i in (0..Self::NDIM).rev() {
            indexes[i] = index % shape[i];
            index /= shape[i];
        }
        indexes
    }
    fn index_wrap<S: Into<i64>>(&self, index: S) -> &Self::Output;
    fn index_wrap_mut<S: Into<i64>>(&mut self, index: S) -> &mut Self::Output;
}

trait NumArray<T, U: Into<usize>>: Array<T, U> + Add<Self> + Sub<Self> + Sized {
    fn zeros() -> Self::ArrayOutput;
    fn ones() -> Self::ArrayOutput;
    fn full(value: <Self as Index<U>>::Output) -> Self::ArrayOutput;
}

struct CompatibleSize<const DATA: bool> {}

trait IsCompatible {}

impl IsCompatible for CompatibleSize<true> {}


trait IsComp1D<const D1: usize, const D2: usize> {
    const RESULT: ();
}

impl <T, const D1: usize, const D2: usize> IsComp1D<D1, D2> for Array1D<T, D1> {
    const RESULT: () = concat_assert!((D1 == D2) | (D1 == 1) | (D2 == 1),
        "\nShape mismatch! For each axis both arrays should have the same size or one of them should be 1.\n",
        "Shape array 1: (", D1,
        ",), Shape array 2: (", D2, ",) \n");
}


macro_rules! mk_array_impl {
    ($val: literal) => {
        mk_array!($val);
        mk_impl_clone!($val);
        mk_impl_index!($val);
        mk_impl_index_mut!($val);
    }
}

mk_over_nums!(mk_array_impl!($1); #5);

pub const fn biggest(a: usize, b: usize) -> usize {
    // check_size_1d(a, b);
    if a > b {
        a
    } else {
        b
    }
}

impl<T: Clone + Add<T, Output=T>, const D1: usize, const D2: usize> Add<Array1D<T, D2>> for Array1D<T, D1>
    where [(); biggest(D1, D2)]:,
{
    type Output = Array1D<T, { biggest(D1, D2) } > ;


    fn add(self, other: Array1D<T, D2>) -> Array1D<T, { biggest(D1, D2) }> {
        let _ = <Array1D<T, D1> as IsComp1D<D1, D2>>::RESULT;
        let data: [T; biggest(D1, D2)] = if D1 == D2 {
            core::array::from_fn(| i | self.data[i].clone() + other.data[i].clone())
        } else if D1 == 1 {
            core::array::from_fn(| i | self.data[0].clone() + other.data[i].clone())
        } else {
            core::array::from_fn(| i | self.data[i].clone() + other.data[0].clone())
        };
        Array1D{data}
    }
}

const fn correct_size<const D1: usize, const D2: usize>() -> () {
    concat_assert!((D1 == D2) | (D1 == 1) | (D2 == 1),
        "Size mismatch! Both axis should have the same size or one of them should be 1. D1: ", D1,
        " D2: ", D2, " ");
}


impl<T: Clone + Sub<T, Output=T>, const D1: usize, const D2: usize> Sub<Array1D<T, D2>> for Array1D<T, D1>
    where [(); biggest(D1, D2)]:
{
    type Output = Array1D<T, { biggest(D1, D2) }>;

    fn sub(self, other: Array1D<T, D2>) -> Array1D<T, { biggest(D1, D2) }> {
        let data: [T; biggest(D1, D2)] = if D1 == D2 {
            core::array::from_fn(| i | self.data[i].clone() - other.data[i].clone())
        } else if D1 == 1 {
            core::array::from_fn(| i | self.data[0].clone() - other.data[i].clone())
        } else {
            core::array::from_fn(| i | self.data[i].clone() - other.data[0].clone())
        };
        Array1D{data}
    }
}


impl<T: Clone + Add<T, Output=T>, const D1: usize, const D2: usize, const D3: usize, const D4: usize> Add<Array2D<T, D3, D4>> for Array2D<T, D1, D2>
    where CompatibleSize<{ ((D1 == D3) | (D1 == 1) | (D2 ==1)) & ((D2==D4) | (D2 == 1) | (D4==1)) }>: IsCompatible,
          [(); biggest(D1, D3)*biggest(D2, D4)]:,
          [(); D1*D2]:,
          [(); D3*D4]:
{
    type Output = Array2D<T, {biggest(D1, D3)}, {biggest(D2, D4)}>;

    fn add(self, other: Array2D<T, D3, D4>) -> Array2D<T, {biggest(D1, D3)}, {biggest(D2, D4)}> {
        if D1 == D3 && D2 == D4 {
            let data: [T; biggest(D1, D3) * biggest(D2, D4)] = core::array::from_fn(|i| self.data[i].clone() + other.data[i].clone());
            Array2D { data }
        }
        else {
            todo!()
        }
    }
}