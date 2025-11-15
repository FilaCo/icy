use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn derive(input: &DeriveInput) -> TokenStream {
    let generics = &input.generics;
    let ident = &input.ident;
    quote! {
        impl #generics yadi::prelude::Injectable for #ident #generics {
            fn from_registry(registry: &yadi::prelude::Registry) -> Self {

            }
        }
    }
}
