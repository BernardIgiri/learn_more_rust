extern crate proc_macro;

use proc_macro::TokenStream;
use syn;
use quote::quote;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote!{
        impl Hello for #name {
            fn hello() -> String {
                format!("Hello, Macro! My name is {}", stringify!(#name)).into()
            }
        }
    };
    gen.into()
}
