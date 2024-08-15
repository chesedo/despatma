#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use syn::parse_macro_input;
use visitor::VisitorFunction;

mod visitor;

#[proc_macro]
pub fn visitor(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as VisitorFunction);

    input.expand().into()
}
