use proc_macro::{TokenStream};
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use syn::parse::Parse;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro]
pub fn test_macro(input: TokenStream) -> TokenStream {
    // If the first value in the input is bigger than 10, make an compile error
    let input = parse_macro_input!(input as syn::LitInt);
    if input.base10_parse::<u32>().unwrap() > 10 {
        return syn::Error::new(Span::call_site(), "Value is bigger than 10").to_compile_error().into();
    };
    quote! {
        println!("Value is smaller than 10");
    }.into()
}

