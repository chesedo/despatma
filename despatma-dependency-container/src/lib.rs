#![doc = include_str!("../README.md")]

use container::Container;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::{parse_macro_input, ItemImpl};

mod container;
mod visitor;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn dependency_container(_tokens: TokenStream, impl_expr: TokenStream) -> TokenStream {
    let input = parse_macro_input!(impl_expr as ItemImpl);
    let mut container = Container::from_item_impl(input);

    container.validate();
    container.update();

    quote! {
        #container
    }
    .into()
}
