extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Dirty)]
pub fn dirty_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_dirty(&ast)
}


fn impl_dirty(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let gen = quote! {
        impl #generics crate::macro_traits::Dirty for #name #generics
            where #name: Clone + Eq + PartialEq
         {
            fn dirty() -> bool {
                true
            }
        }
    };
    gen.into()
}



#[proc_macro_derive(BuilderExt)]
pub fn builder_ext_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_builder_ext(&ast)
}

#[proc_macro_derive(ReaderExt)]
pub fn reader_ext_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_reader_ext(&ast)
}


fn impl_builder_ext(ast: &syn::DeriveInput) -> TokenStream {
    use proc_macro2::{Ident, Span};

    let name = &ast.ident;
    let generics = &ast.generics;

    //let data_name = Ident::new(&format!("{}Data", name), Span::call_site());

    let gen = quote! {
        impl #generics crate::macro_traits::BuilderExt for #name #generics {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}

fn impl_reader_ext(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;

    let gen = quote! {
        impl #generics crate::macro_traits::ReaderExt for #name #generics {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}
