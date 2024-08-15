#![doc = include_str!("../README.md")]

use abstract_factory::AbstractFactoryAttribute;
use despatma_lib::TraitSpecifier;
use proc_macro::TokenStream;
use syn::{parse_macro_input, punctuated::Punctuated, ItemTrait, Token};
use tokenstream2_tmpl::Interpolate;

mod abstract_factory;

#[proc_macro_attribute]
pub fn abstract_factory(tokens: TokenStream, trait_expr: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(trait_expr as ItemTrait);
    let attributes = parse_macro_input!(tokens as AbstractFactoryAttribute);

    attributes.expand(&mut input).into()
}

#[proc_macro_attribute]
pub fn interpolate_traits(tokens: TokenStream, concrete_impl: TokenStream) -> TokenStream {
    let attributes =
        parse_macro_input!(tokens with Punctuated::<TraitSpecifier, Token![,]>::parse_terminated);

    attributes.interpolate(concrete_impl.into()).into()
}
