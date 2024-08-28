pub(crate) const fn max<const D1: usize, const D2: usize>() -> usize {
    if D1 > D2 {
        D1
    } else {
        D2
    }
}

pub(crate) const fn max2(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}