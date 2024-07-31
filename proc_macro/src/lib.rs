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

/// This macro generates the implementation of a math operator trait for an ArrayND type.
/// The input is a comma seperated string of the form:
/// trait_name, func_name, parameter_name, num
#[proc_macro]
pub fn mk_impl_math(input: TokenStream) -> TokenStream {
    let values = input.to_string();
    let values: Vec<&str> = values.split(",").collect();
    if values.len() != 4 {
        panic!("Expected 4 comma seperated values, got {}", values.len());
    }
    let trait_name = values[0];
    let func_name = values[1];
    let parameter_name = values[2];
    let num = values[3].trim().parse::<usize>().expect("Expected a usize as the last argument");

    let output = format!("\
    impl<T: {trait_name}<T, Output=T> + Clone, {const_D}> {trait_name}<T> for Array{num}D<T, {d_vals}> where [(); {d_mul}]:,
        {{
            type Output = Array{num}D<T, {d_vals}>;

            /// This function performs the operation {func_name} on the array, it does not mutate the array.
            fn {func_name}(self, {parameter_name}: T) -> Self::Output {{
                let data = core::array::from_fn(|i| self.data[i].clone().{func_name}({parameter_name}.clone()));
                Array{num}D{{data}}
            }}
        }}
    ", const_D=const_x_vals_str(num, "D"), d_vals=x_vals_str(num, "D"), d_mul=x_mult_str(num, "D"));
    output.parse().unwrap()
}

/// This macro generates the implementation of an inplace math trait for an ArrayND type.
/// The input is a comma seperated string of the form:
/// trait_name, func_name, parameter_name, num
#[proc_macro]
pub fn mk_impl_math_inplace(input: TokenStream) -> TokenStream {
    let values = input.to_string();
    let values: Vec<&str> = values.split(",").collect();
    if values.len() != 4 {
        panic!("Expected 4 comma seperated values, got {}", values.len());
    }
    let trait_name = values[0];
    let func_name = values[1];
    let parameter_name = values[2];
    let num = values[3].trim().parse::<usize>().expect("Expected a usize as the last argument");

    let output = format!("\
    impl<T: {trait_name} + Clone, {const_D}> {trait_name}<T> for Array{num}D<T, {d_vals}> where [(); {d_mul}]:,
        {{
            /// This function performs the inplace operation {func_name} on the array, it mutates the array.
            fn {func_name}(&mut self, {parameter_name}: T) {{
                for i in 0..({d_mul}) {{
                    self.data[i].{func_name}({parameter_name}.clone());
                }}
            }}
        }}
    ", const_D=const_x_vals_str(num, "D"), d_vals=x_vals_str(num, "D"), d_mul=x_mult_str(num, "D"));
    output.parse().unwrap()
}

#[proc_macro]
pub fn mk_impl_reshape(input: TokenStream) -> TokenStream {
    let values = input.to_string();
    let values: Vec<&str> = values.split("to").collect();
    if values.len() != 2 {
        panic!("Expected 2 values seperated by 'to', got {}", values.len());
    }
    let from = values[0].trim().parse::<usize>().expect("Expected a usize as the first argument");
    let to = values[1].trim().parse::<usize>().expect("Expected a usize as the second argument");
    let output = format!("\
    impl<T: Clone, {d_from_const}> Array{from}D<T, {d_vals}> where [(); {d_mul}]:,
    {{
        /// This function reshapes the array to a new shape.
        pub fn reshape_to_{to}d<{d_to_const}>(&self) -> Array{to}D<&T, {to_vals}> where [(); {d_to_mul}]:,
        {{
            let data = core::array::from_fn(|i| &self.data[i]);
            Array{to}D{{data}}
        }}
    }}", d_from_const=const_x_vals_str(from, "D1_"), from=from, d_to_const=const_x_vals_str(to, "D2_"),
         d_vals=x_vals_str(from, "D1_"), d_mul=x_mult_str(from, "D1_"), to=to, to_vals=x_vals_str(to, "D2_"),
         d_to_mul=x_mult_str(to, "D2_"));
    output.parse().unwrap()
}

/// This macro replaces the `${i}` in the first argument with 1 to the value after the `i`th `#`.
/// The first argument cannot contain any `#`.
///
/// Example:
/// mk_over_nums!("$1_$2; #2 #3") -> $1_1;\n $1_2;\n $1_3;\n $2_1;\n $2_2;\n $2_3;\n
#[proc_macro]
pub fn mk_over_nums(input: TokenStream) -> TokenStream {
    let values = input.to_string();

    let mut values: Vec<&str> = values.split(" #").collect();
    let mut output = values[0].to_owned();
    output.push_str("\n");
    values.remove(0);

    for i in 1..=values.len() {
        let mut temp = "".to_owned();
        if !output.contains(&format!("${}", i)) {
            panic!("Expected at least one ${} in the first argument to replace with the index", i);
        }
        let num = values[i-1].trim().parse::<usize>().expect(&format!("Expected a usize as argument {}", i));
        for j in 1..=num {
            temp.push_str(&output.replace(&format!("${}", i), &j.to_string()));
        }
        output = temp;
    }
    println!("{:?}", output);
    output.parse().unwrap()
}

