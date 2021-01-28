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
