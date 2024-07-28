#![feature(generic_const_exprs)]
mod array;
mod indexer;
mod ops;
mod constructors;
mod maths;


#[cfg(test)]
mod tests {
    use crate::array::*;
    use crate::indexer::*;

    #[test]
    fn test_array1d() {
        let mut a = Array1D{data: [1, 2, 3]};
        assert_eq!(a[0], 1);
        assert_eq!(a[1], 2);
        assert_eq!(a[2], 3);
        a[0] = 4;
        assert_eq!(a[0], 4);
    }

    #[test]
    fn test_array1d_add() {
        let a = Array1D{data: [1, 2, 3]};
        let b = Array1D{data: [4, 5, 6]};
        let c = a.clone() + b;
        assert_eq!(c[0], 5);
        assert_eq!(c[1], 7);
        assert_eq!(c[2], 9);

        let d = Array1D{data: [7]};
        let e = d + a.clone();
        assert_eq!(e[0], 8);
        assert_eq!(e[1], 9);
        assert_eq!(e[2], 10);

        // let _ = a[Indexer1D::<5>{}];

        let e = Array1D{data: [[9, 2], [3, 4]]};
        let f = e  + a.clone();
    }
}
