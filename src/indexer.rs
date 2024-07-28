use std::ops::Index;
use const_panic::concat_assert;
use crate::array::{Array1D, Array2D, Array3D, Array4D, Array5D};
use proc_macro::{mk_indexer, mk_indexer_trait, mk_indexer_impl};


macro_rules! mk_values {
    ($val: literal) => {
        mk_indexer!($val);
        mk_indexer_trait!($val);
        mk_indexer_impl!($val);
    };
}

mk_values!(1);
mk_values!(2);
mk_values!(3);
mk_values!(4);
mk_values!(5);
