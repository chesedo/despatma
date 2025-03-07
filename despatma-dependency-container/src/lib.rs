#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro_error2::proc_macro_error;
use quote::quote;
use syn::{parse_macro_input, ItemImpl};

mod input;
mod output;
mod processing;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn dependency_container(_tokens: TokenStream, impl_expr: TokenStream) -> TokenStream {
    let input = parse_macro_input!(impl_expr as ItemImpl);
    let input = input::Container::from_item_impl(input);
    let mut processing: processing::Container = input.into();
    processing.process();
    let output: output::Container = processing.into();

    quote! {
        #output
    }
    .into()
}
