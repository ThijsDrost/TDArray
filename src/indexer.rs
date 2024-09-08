use std::ops::Index;
// use std::format;
use std::{concat, stringify};
use const_panic::concat_assert;
use crate::array::{Array1D, Array2D, Array3D, Array4D, Array5D};
use proc_macro::{mk_indexer, mk_indexer_trait, mk_indexer_impl, proc_const_x_vals_str};
use paste::paste;

macro_rules! mk_indexern {
    ($val: literal) => {
        paste! {
            pub struct [<Indexer $val D>]< >{}
        }
    };
}

macro_rules! mk_indexern2 {
    ($val: literal, $($val2: tt)+) => {
        paste! {
            pub struct [<Indexer $val D>]<$($val2)+> {}
        }
    };
}

macro_rules! mk_indexern3 {
    ($val: literal, $($val2: tt)+) => {
        paste! {
            pub struct [<Indexer $val D>]
        }
        < $($val2)+ > {}
    };
}

pub fn pre_post_vals(prefix: &str, postfix: &str, num: usize) -> String {
    let mut output = format!("{prefix}1{postfix}", prefix=prefix, postfix=postfix);
    for i in 2..=num {
        output.push_str(&format!(", {prefix}{i}{postfix}", prefix=prefix, i=i, postfix=postfix));
    }
    output
}

pub fn const_x_vals_str(num: usize, name: &str) -> String {
    pre_post_vals(&format!("const {}", name), ": usize", num)
}



mk_indexern!(1);
mk_indexern2!(2, const I1: usize);
mk_indexern3!(2, const I1: usize);

// proc_const_x_vals_str!{I; 10};
fn test() {
    let value = const_x_vals_str(1, "I");
    // proc_const_x_vals_str!(I; 1) = 10;
}


// macro_rules! mk_values {
//     ($val: literal) => {
//         mk_indexer!($val);
//         mk_indexer_trait!($val);
//         mk_indexer_impl!($val);
//     };
// }
//
// mk_values!(1);
// mk_values!(2);
// mk_values!(3);
// mk_values!(4);
// mk_values!(5);
