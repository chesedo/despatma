//! Library for the [despatma] crate. This library has some extra tokens not defined in [syn] that are used by
//! [despatma]. These are used as options for design pattern inputs or for type inputs.
//!
//! [syn]: https://github.com/dtolnay/syn
//! [despatma]: https://github.com/chesedo/despatma
//!
//! # Optional features
//! Like [syn], some functionality are behind optional features to optimize compile-time. Currently the follow feature
//! is available:
//! - `extra-traits` — Debug, Eq, PartialEq, Hash impls for all syntax tree types.

mod annotated_type;
pub mod extensions;
mod key_value;
mod options_attribute;
mod simple_type;
mod trait_specifier;

pub use annotated_type::AnnotatedType;
pub use key_value::KeyValue;
pub use options_attribute::OptionsAttribute;
pub use simple_type::SimpleType;
pub use trait_specifier::TraitSpecifier;
// TODO: consider if some Punctuated::parse_terminated should no be Punctuated::parse_seperated_nonempty
