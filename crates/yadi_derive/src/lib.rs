use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod injectable;

#[proc_macro_derive(Injectable, attributes(injector))]
pub fn derive_injectable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    injectable::derive(&input).into()
}

