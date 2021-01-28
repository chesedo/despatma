mod abstract_factory;
mod visitor;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, ItemTrait, Token};
use tokenstream2_tmpl::Interpolate;

use abstract_factory::AbstractFactoryAttribute;
use despatma_lib::TraitSpecifier;
use visitor::VisitorFunction;

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

#[proc_macro]
pub fn visitor(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as VisitorFunction);

    input.expand().into()
}
