mod string_funcs;

use proc_macro::TokenStream;
use std::ops::{Index, IndexMut};
use string_funcs::*;

#[proc_macro]
pub fn mk_indexer(input: TokenStream) -> TokenStream {
    let num = input.to_string().parse::<usize>().unwrap();
    let output =
        format!("pub struct Indexer{}D<{}>{{}}", num, const_x_vals_str(num, "I"));
    output.parse().unwrap()
}

#[proc_macro]
pub fn mk_indexer_trait(input: TokenStream) -> TokenStream {
    let num = input.to_string().parse::<usize>().unwrap();
    let mut output =
        format!("trait Index{}D<{}>{{const RESULT: ();}}", num, const_DI_vals_str(num));
    output.parse().unwrap()
}

#[proc_macro]
pub fn mk_indexer_impl(input: TokenStream) -> TokenStream {
    let num = input.to_string().parse::<usize>().unwrap();

    fn concat_assert(num: usize) -> String {
        let mut output = String::new();
        for i in 1..=num {
            output.push_str(&format!("\nconcat_assert!((D{i} > I{i}), \"\nIndex out of bounds! Array length: \", D{i}, \", Index: \", I{i}, \" \n\");"));
        }
        output
    }

    fn calc_index(num: usize) -> String {
        let mut output = "I1".to_owned();
        for i in 2..=num {
            output.push_str(&format!(" + I{i}*D{}", i-1));
        }
        output
    }

    let mut output = format!("\
    impl <T, {}> {} for {} \n
    where [(); {}]:, \n\
    {{\n\t\
        const RESULT: () = {{\n\
            {}\n\
        }};\n\
    }}\n", const_DI_vals_str(num), index_str(num), array_str(num), x_mult_str(num, "D"), concat_assert(num));

    let output2 = format!("\
    impl<T, {}> Index<Indexer{}D<{}>> for {}\n\t\
        where [(); {}]:, \n\
    {{\n\t\
        type Output = T;\n\n\t\
        fn index(&self, index: Indexer{}D<{}>) -> &T {{\n\t\t\
            let _ = <{} as Index{}D<{}>>::RESULT;\n\t\t\
            &self.data[{}]}}\n\
    }}\n",
                          const_DI_vals_str(num), num, x_vals_str(num, "I"), array_str(num), x_mult_str(num, "D"), num,
                          x_vals_str(num, "I"), array_str(num), num, DI_vals_str(num), calc_index(num));

    output.push_str(&output2);
    output.parse().unwrap()
}

#[proc_macro]
pub fn mk_array(input: TokenStream) -> TokenStream {
    let num = input.to_string().parse::<usize>().unwrap();
    let output =
        format!("pub struct Array{}D<T, {}>  where [(); {d_mult}]:, {{\
        pub data: [T; {d_mult}],\
        }}", num, const_x_vals_str(num, "D"), d_mult= x_mult_str(num, "D"));
    output.parse().unwrap()
}

#[proc_macro]
pub fn mk_impl_clone(input: TokenStream) -> TokenStream {
    let num = input.to_string().parse::<usize>().unwrap();
    let output = format!("impl<T, {}> Clone for Array{}D<T, {}> where T: Clone, [(); {}]: {{\n\t\
        fn clone(&self) -> Self {{\n\t\t\
            Array{}D{{data: self.data.clone()}}\n\t\
        }}\n\
    }}", const_x_vals_str(num, "D"), num, x_vals_str(num, "D"), x_mult_str(num, "D"), num);
    output.parse().unwrap()

}

#[proc_macro]
pub fn mk_impl_index(input: TokenStream) -> TokenStream {
    let num = input.to_string().parse::<usize>().unwrap();
    let output = format!("impl<T, {}> Index<usize> for Array{}D<T, {}> where [(); {}]: {{\n\t\
        type Output = T;\n\n\t\
        fn index(&self, index: usize) -> &T {{\n\t\t\
            &self.data[index]\n\t\
        }}\n\
    }}", const_x_vals_str(num, "D"), num, x_vals_str(num, "D"), x_mult_str(num, "D"));
    output.parse().unwrap()
}

#[proc_macro]
pub fn mk_impl_index_mut(input: TokenStream) -> TokenStream {
    let num = input.to_string().parse::<usize>().unwrap();
    let output = format!("impl<T, {}> IndexMut<usize> for Array{}D<T, {}> where [(); {}]: {{\n\t\
        fn index_mut(&mut self, index: usize) -> &mut T {{\n\t\t\
            &mut self.data[index]\n\t\
        }}\n\
    }}", const_x_vals_str(num, "D"), num, x_vals_str(num, "D"), x_mult_str(num, "D"));
    output.parse().unwrap()
}

// impl <T, const D1: usize, const D2: usize> IsComp1D<D1, D2> for Array1D<T, D1> {
//     const RESULT: () = concat_assert!((D1 == D2) | (D1 == 1) | (D2 == 1),
//         "\nShape mismatch! For each axis both arrays should have the same size or one of them should be 1.\n",
//         "Shape array 1: (", D1,
//         ",), Shape array 2: (", D2, ",) \n");
// }

#[proc_macro]
pub fn mk_is_comp_nd(input: TokenStream) -> TokenStream {
    let num = input.to_string().parse::<usize>().unwrap();
    let output = format!(
        "trait IsComp{n}D<{d1}, {d2}> {{const RESULT: ();}}\
        \
        impl <T, {d1}, {d2}> IsComp{n}D<{d1}, {d2}> for Array{n}D<T, {d1}> where: [(); {d1_mult}]:,\
        {{\
            const RESULT: () = {{\
                concat_assert!({eq_test},\
                \"\nShape mismatch! For each axis both arrays should have the same size or one of them should be 1.\n\",\
                \"Shape array 1: (\", {d1},\
                \",), Shape array 2: (\", {d2}, \",) \n\");\
            }};\
        }}",
        n=num, d1=const_x_vals_str(num, "D1_"), d2=const_x_vals_str(num, "D2_"),
        d1_mult=x_mult_str(num, "D1_"), eq_test=x_eq_vals_str(num, "D1_", "D2_"));
    output.parse().unwrap()
}

#[proc_macro]
pub fn mk_from_vec(input: TokenStream) -> TokenStream {
    let num = input.to_string().parse::<usize>().unwrap();
    let output = format!(
        "impl<T> From<Vec<T>> for Array{n}D<T, 1> where [(); 1]: {{\
            fn from(vec: Vec<T>) -> Self {{\
                let mut data = core::array::from_fn(|i| vec[i]);\
                Array{n}D{{data}}\
            }}\
        }}",
        n=num);
    output.parse().unwrap()
}
