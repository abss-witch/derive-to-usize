//! Derive for From and Into usize
//! ```
//! use derive_into::ToUsize;
//! #[derive(ToUsize)]
//! enum Foo {
//!    Bar,
//!    E
//! }
//! assert_eq!(0_usize, Foo::Bar.into());
//! assert_eq!(1_usize, Foo::E.into());
//! ```



use quote::{quote};
#[proc_macro_derive(ToUsize)]
pub fn to_usize(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let name = input.ident;
    let (impl_gen, ty_gen, ..) = input.generics.split_for_impl();
    proc_macro::TokenStream::from(quote! {
        impl #impl_gen std::convert::From<#name #ty_gen> for usize {
            fn from(value: #name #ty_gen) -> usize {
                value as usize
            }
        }
    })

}

