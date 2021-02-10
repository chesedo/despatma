//! Despatma is a collection of `des`ign `pat`tern `ma`cros (`despatma`).
//! It aims to provide the most common implementations for design patterns at run-time.
//!
//! This project is still a **work in progress**.
//! The end goal is to be as [Loki](http://loki-lib.sourceforge.net/) is for C++ and more if possible.
//! The following patterns are currently implemented:
//! - [abstract_factory] - with the help of [interpolate_traits] macro
//! - [visitor]
//!
//! [abstract_factory]: macro@self::abstract_factory
//! [interpolate_traits]: macro@self::interpolate_traits
//! [visitor]: macro@self::visitor
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

/// Creates an abstract visitor for a list of elements.
///
/// This macro does three things:
/// 1. A `Visitor` trait is created with methods to visit each element. Each method calls a default helper function by default.
/// 1. A helper function is created for each element. The idea is for this function to transverse into the elements children.
/// 1. A `Visitable` trait is created that redirects / reflects each element back to its visitor
///
/// # Example input
/// ```
/// use despatma::visitor;
///
/// visitor!(
///     dyn Arc,
///     Rectangle,
///     dyn Point,
/// );
/// ```
///
/// ## Output
/// The three elements listed above will be created.
/// ```
/// use despatma::visitor;
///
/// pub trait Visitor {
///     fn visit_arc(&mut self, arc: &dyn Arc) {
///         visit_arc(self, arc)
///     }
///     fn visit_rectangle(&mut self, rectangle: &Rectangle) {
///         visit_rectangle(self, rectangle)
///     }
///     fn visit_point(&mut self, point: &dyn Point) {
///         visit_point(self, point)
///     }
/// }
///
/// pub fn visit_arc<V>(visitor: &mut V, _arc: &dyn Arc)
/// where
///     V: Visitor + ?Sized,
/// {
/// }
/// pub fn visit_rectangle<V>(visitor: &mut V, _rectangle: &Rectangle)
/// where
///     V: Visitor + ?Sized,
/// {
/// }
/// pub fn visit_point<V>(_visitor: &mut V, _point: &dyn Point)
/// where
///     V: Visitor + ?Sized,
/// {
/// }
///
/// trait Visitable {
///     fn apply(&self, visitor: &mut dyn Visitor);
/// }
/// impl Visitable for dyn Arc {
///     fn apply(&self, visitor: &mut dyn Visitor) {
///         visitor.visit_arc(self);
///     }
/// }
/// impl Visitable for Rectangle {
///     fn apply(&self, visitor: &mut dyn Visitor) {
///         visitor.visit_rectangle(self);
///     }
/// }
/// impl Visitable for dyn Point {
///     fn apply(&self, visitor: &mut dyn Visitor) {
///         visitor.visit_point(self);
///     }
/// }
/// ```
///
/// The input shows `Visitor` can be applied to structs (`Rectangle`) and traits (`Arc` and `Point`).
///
/// ## Usage
/// Any visitor can now just implement the `Visitor` trait and provide its own implementation for any of the visitor methods.
/// ```
/// use my_lib::{Visitor, visit_point};
///
/// struct PointCounter {
///     pub count: usize,
/// }
///
/// impl Visitor for PointCounter {
///     fn visit_point(&mut self, point: &dyn Point) {
///         self.count += 1;
///
///         // Call helper function to keep transversal intact
///         visit_point(self, point)
///     }
/// }
/// ```
///
/// This visitor will now count all the points in a hierarchy.
/// But there is a problem. The default helper implementations do not have any transversal code. This can be fixed with the `helper_tmpl` option.
///
/// ## `helper_tmpl` option
/// This option will fill the body of the helper method with the given code.
/// ```
/// use despatma::visitor;
///
/// visitor!(
///     #[helper_tmpl = {visitor.visit_point(arc.center);} ]
///     dyn Arc,
///
///     #[
///         helper_tmpl = {
///             visitor.visit_point(rectangle.top_left);
///             visitor.visit_point(rectangle.bottom_right);
///         },
///     ]
///     Rectangle,
///
///     dyn Point,
/// );
/// ```
///
/// The helper functions will now look as follow:
/// ```
/// // `Visitor` is same as earlier
///
/// pub fn visit_arc<V>(visitor: &mut V, arc: &dyn Arc)
/// where
///     V: Visitor + ?Sized,
/// {
///     visitor.visit_point(arc.center);
/// }
/// pub fn visit_rectangle<V>(visitor: &mut V, rectangle: &Rectangle)
/// where
///     V: Visitor + ?Sized,
/// {
///     visitor.visit_point(rectangle.top_left);
///     visitor.visit_point(rectangle.bottom_right);
/// }
/// pub fn visit_point<V>(_visitor: &mut V, _point: &dyn Point)
/// where
///     V: Visitor + ?Sized,
/// {
/// }
///
/// // `Visitable` is same as earlier
/// ```
///
/// `PointCounter` will now work as expected!
///
/// ## `no_defualt` option
/// You might want to force visitors to implement a visit method and not have a trait default. The default trait implementation can be removed using the `no_default` option.
/// ```
/// use despatma::visitor;
///
/// visitor!(
///     #[
///         helper_tmpl = {visitor.visit_point(arc.center);},
///         no_default,
///     ]
///     dyn Arc,
///
///     #[
///         helper_tmpl = {
///             visitor.visit_point(rectangle.top_left);
///             visitor.visit_point(rectangle.bottom_right);
///         },
///     ]
///     Rectangle,
///
///     dyn Point,
/// );
/// ```
///
/// The `Visitor` trait will now be as follow and `PointCounter` will have to implement the `visit_arc()` method too.
/// ```
/// pub trait Visitor {
///     fn visit_arc(&mut self, arc: &dyn Arc);
///     fn visit_rectangle(&mut self, rectangle: &Rectangle) {
///         visit_rectangle(self, rectangle)
///     }
///     fn visit_point(&mut self, point: &dyn Point) {
///         visit_point(self, point)
///     }
/// }
///
/// // Rest is same as earlier
/// ```
///
/// # Calling a visitor
/// Suppose the follow code exists
/// ```
/// // Create a rectangle with bottom-left point (0, 0) and top-right point (10, 12)
/// let rect = Rectangle::new(0, 0, 10, 12);
/// let point_stats = PointCounter{};
///
/// // Invoke visitor on hierarchy options
/// rect.apply(&mut dyn point_stats); // 1 - Preferred
/// visit_rectangle(&mut point_stats, &rect); // 2
/// point_stats.visit_rectangle(&dyn rect); // 3
/// ```
///
/// The visitor (`PointCounter`) can be invoked in three ways
/// 1. This is the preferred way as it will work with any visitor and there is no need to remember the visit method's name.
/// 1. Needs to know the helper function name and is less generic. But it might work with any visitor.
/// 1. Least generic and also needs to know the method name.
#[proc_macro]
pub fn visitor(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as VisitorFunction);

    input.expand().into()
}
