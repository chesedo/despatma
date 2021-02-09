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

/// Turns a `trait` into an Abstract Factory.
/// The trait can optionally have super-traits.
///
/// This macro is generally used with the [interpolate_traits] macro.
///
/// [interpolate_traits]: macro@self::interpolate_traits
///
/// # Example input
/// A factory method needs to be defined to use this macro. The factory method is passed as the first argument to the macro. The rest of the arguments passed to the macro are the elements the factory will create.
/// ```
/// use despatma::abstract_factory;
///
/// // A factory method
/// // `Element` is a trait defined by you.
/// pub trait Factory<T: Element + ?Sized> {
///     fn create(&self, name: String) -> Box<T>;
/// }
///
/// // The abstract factory that also needs to implement `Display` and `Eq`.
/// // The factory method above (`Factory`) is the first input.
/// #[abstract_factory(Factory, dyn Button, dyn Scroller, dyn Window)]
/// pub trait GuiFactory: Display + Eq {}
/// ```
///
/// ## Output
/// This will create the following code
/// ```
/// use despatma::abstract_factory;
///
/// // A factory method
/// // `Element` is a trait defined by you.
/// pub trait Factory<T: Element + ?Sized> {
///     fn create(&self, name: String) -> Box<T>;
/// }
///
/// // The abstract factory that also needs to implement `Display` and `Eq`.
/// // The factory method above (`Factory`) is the first input.
/// #[abstract_factory(Factory, dyn Button, dyn Scroller, dyn Window)]
/// pub trait GuiFactory:
///     Display
///     + Eq
///     + Factory<dyn Button>
///     + Factory<dyn Scroller>
///     + Factory<dyn Window>
/// {
/// }
/// ```
#[proc_macro_attribute]
pub fn abstract_factory(tokens: TokenStream, trait_expr: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(trait_expr as ItemTrait);
    let attributes = parse_macro_input!(tokens as AbstractFactoryAttribute);

    attributes.expand(&mut input).into()
}

/// Expands a list of [despatma_lib::TraitSpecifier] elements over a template.
/// The `TRAIT` and `CONCRETE` markers in the template will be replaced with each passed in element.
/// The template is annotated with this attribute.
///
/// This macro can be used to create concrete implementations for [abstract_factory].
///
/// # Example input
/// ```
/// use despatma::interpolate_traits;
///
/// // GuiFactory and Factory is defined in the abstract_factory example
/// struct GnomeFactory{}
/// impl GuiFactory for GnomeFactory{}
///
/// // Implement the factory method for each GUI element
/// #[interpolate_traits(
///     Button => GnomeButton,
///     Scroller => Gnome2Scroller,
///     Window => Gnome3Window,
/// )]
/// impl Factory<dyn TRAIT> for GnomeFactory {
///     fn create(&self, name: String) -> Box<dyn TRAIT> {
///         Box::new(CONCRETE::new(name))
///     }
/// }
/// ```
///
/// ## Output
/// This will implement the factory method (expand the template) for each element as follow.
/// ```
/// use despatma::interpolate_traits;
///
/// // GuiFactory and Factory is defined in the abstract_factory example
/// struct GnomeFactory{}
/// impl GuiFactory for GnomeFactory{}
///
/// // Implement the factory method for each GUI element
/// impl Factory<dyn Button> for GnomeFactory {
///     fn create(&self, name: String) -> Box<dyn Button> {
///         Box::new(GnomeButton::new(name))
///     }
/// }
/// impl Factory<dyn Scroller> for GnomeFactory {
///     fn create(&self, name: String) -> Box<dyn Scroller> {
///         Box::new(Gnome2Scroller::new(name))
///     }
/// }
/// impl Factory<dyn Window> for GnomeFactory {
///     fn create(&self, name: String) -> Box<dyn Window> {
///         Box::new(Gnome3Window::new(name))
///     }
/// }
/// ```
///
/// [abstract_factory]: macro@self::abstract_factory
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
