//! [![github]](https://github.com/chesedo/despatma)&ensp;[![crates-io]](https://crates.io/crates/despatma)&ensp;[![docs-rs]](https://docs.rs/despatma)&ensp;[![workflow]](https://github.com/chesedo/despatma/actions?query=workflow%3ARust)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//! [workflow]: https://img.shields.io/github/actions/workflow/status/chesedo/despatma/rust.yml?color=green&label=&labelColor=555555&logo=github%20actions&logoColor=white&style=for-the-badge
//!
//! Despatma is a collection of `des`ign `pat`tern `ma`cros (`despatma`).
//! It aims to provide the most common implementations for design patterns at run-time.
//!
//! This project is still a **work in progress**.
//! The end goal is to be as [Loki](http://loki-lib.sourceforge.net/) is for C++ and more if possible.
//! The following patterns are currently implemented:
//! - [abstract_factory] - with the help of [interpolate_traits] macro
//! - [visitor]
//! - [dependency_container]
//!
//! [abstract_factory]: crate::abstract_factory
//! [interpolate_traits]: crate::interpolate_traits
//! [visitor]: crate::visitor
//! [dependency_container]: crate::dependency_container

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
/// use std::fmt::Display;
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
///
/// pub trait Element {
///     fn create() -> Self
///     where
///         Self: Sized;
/// }
///
/// pub trait Button: Element {
///     fn click(&self);
/// }
///
/// pub trait Scroller: Element {
///     fn scroll(&self, x: i32, y: i32);
/// }
///
/// pub trait Window: Element {
///     fn resize(&self, width: u32, height: u32);
/// }
/// ```
///
/// ## Output
/// This will create the following code
/// ```
/// use despatma::abstract_factory;
/// use std::fmt::Display;
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
///
/// pub trait Element {
///     fn create() -> Self
///     where
///         Self: Sized;
/// }
///
/// pub trait Button: Element {
///     fn click(&self);
/// }
///
/// pub trait Scroller: Element {
///     fn scroll(&self, x: i32, y: i32);
/// }
///
/// pub trait Window: Element {
///     fn resize(&self, width: u32, height: u32);
/// }
/// ```
pub use despatma_abstract_factory::abstract_factory;

/// Expands a list of [despatma_lib::TraitSpecifier] elements over a template.
/// The `TRAIT` and `CONCRETE` markers in the template will be replaced with each passed in element.
/// The template itself is annotated with this attribute.
///
/// This macro can be used to create concrete implementations for [abstract_factory].
///
/// # Example input
/// ```
/// use despatma::{abstract_factory, interpolate_traits};
/// use std::fmt::Display;
///
/// // GuiFactory and Factory are explained in the abstract_factory example
/// struct GnomeFactory{}
///
/// pub trait Factory<T: Element + ?Sized> {
///     fn create(&self, name: String) -> Box<T>;
/// }
///
/// #[abstract_factory(Factory, dyn Button, dyn Scroller, dyn Window)]
/// pub trait GuiFactory: Display + Eq {}
///
/// // Implement the factory method for each GUI element
/// #[interpolate_traits(
///     Button => GnomeButton,
///     Scroller => Gnome2Scroller,
///     Window => Gnome3Window,
/// )]
/// impl Factory<dyn TRAIT> for GnomeFactory {
///     fn create(&self, name: String) -> Box<dyn TRAIT> {
///         Box::new(CONCRETE::create(name))
///     }
/// }
///
/// pub trait Element {
///     fn create(name: String) -> Self
///     where
///         Self: Sized;
/// }
/// #[interpolate_traits(
///     Element => GnomeButton,
///     Element => Gnome2Scroller,
///     Element => Gnome3Window,
/// )]
/// impl TRAIT for CONCRETE {
///     fn create(name: String) -> Self {
///         CONCRETE { name }
///     }
/// }
///
/// pub trait Button: Element {
///     fn click(&self);
/// }
///
/// pub trait Scroller: Element {
///     fn scroll(&self, x: i32, y: i32);
/// }
///
/// pub trait Window: Element {
///     fn resize(&self, width: u32, height: u32);
/// }
///
/// struct GnomeButton {
///     name: String,
/// }
/// impl Button for GnomeButton {
///     fn click(&self) {}
/// }
///
/// struct Gnome2Scroller {
///     name: String,
/// }
/// impl Scroller for Gnome2Scroller {
///     fn scroll(&self, _x: i32, _y: i32) {}
/// }
///
/// struct Gnome3Window {
///     name: String,
/// }
/// impl Window for Gnome3Window {
///     fn resize(&self, _width: u32, _height: u32) {}
/// }
/// ```
///
/// ## Output
/// This will implement the factory method (expand the template) for each element as follow.
/// ```
/// use despatma::{abstract_factory, interpolate_traits};
/// use std::fmt::Display;
///
/// // GuiFactory and Factory are explained in the abstract_factory example
/// struct GnomeFactory{}
///
/// pub trait Factory<T: Element + ?Sized> {
///     fn create(&self, name: String) -> Box<T>;
/// }
///
/// #[abstract_factory(Factory, dyn Button, dyn Scroller, dyn Window)]
/// pub trait GuiFactory: Display + Eq {}
///
/// // Implement the factory method for each GUI element
/// impl Factory<dyn Button> for GnomeFactory {
///     fn create(&self, name: String) -> Box<dyn Button> {
///         Box::new(GnomeButton::create(name))
///     }
/// }
/// impl Factory<dyn Scroller> for GnomeFactory {
///     fn create(&self, name: String) -> Box<dyn Scroller> {
///         Box::new(Gnome2Scroller::create(name))
///     }
/// }
/// impl Factory<dyn Window> for GnomeFactory {
///     fn create(&self, name: String) -> Box<dyn Window> {
///         Box::new(Gnome3Window::create(name))
///     }
/// }
///
/// pub trait Element {
///     fn create(name: String) -> Self
///     where
///         Self: Sized;
/// }
/// impl Element for GnomeButton {
///     fn create(name: String) -> Self {
///         GnomeButton { name }
///     }
/// }
/// impl Element for Gnome2Scroller {
///     fn create(name: String) -> Self {
///         Gnome2Scroller { name }
///     }
/// }
/// impl Element for Gnome3Window {
///     fn create(name: String) -> Self {
///         Gnome3Window { name }
///     }
/// }
///
/// pub trait Button: Element {
///     fn click(&self);
/// }
///
/// pub trait Scroller: Element {
///     fn scroll(&self, x: i32, y: i32);
/// }
///
/// pub trait Window: Element {
///     fn resize(&self, width: u32, height: u32);
/// }
///
/// struct GnomeButton {
///     name: String,
/// }
/// impl Button for GnomeButton {
///     fn click(&self) {}
/// }
///
/// struct Gnome2Scroller {
///     name: String,
/// }
/// impl Scroller for Gnome2Scroller {
///     fn scroll(&self, _x: i32, _y: i32) {}
/// }
///
/// struct Gnome3Window {
///     name: String,
/// }
/// impl Window for Gnome3Window {
///     fn resize(&self, _width: u32, _height: u32) {}
/// }
/// ```
///
/// [abstract_factory]: macro@self::abstract_factory
pub use despatma_abstract_factory::interpolate_traits;

/// Creates an abstract visitor for a list of elements.
///
/// This macro does three things:
/// 1. A `Visitor` trait is created with methods to visit each element in the list. Each method calls a default helper function by default (see point 2).
/// 1. A helper function is created for each element. The idea is for this function to traverse into the elements children.
/// 1. A `Visitable` trait is created that redirects / reflects each element back to its visitor
///
/// # Example input
/// ```
/// use despatma::visitor;
///
/// visitor!(
///     Arc,
///     Rectangle,
///     Point,
/// );
///
/// struct Arc {
///    center: Point,
///    radius: u32,
/// }
///
/// struct Rectangle {
///     top_right: Point,
///     bottom_left: Point,
/// }
///
/// struct Point {
///     x: i32,
///     y: i32,
/// }
/// ```
///
/// ## Output
/// The three sets of things listed earlier will be created.
/// ```
/// use despatma::visitor;
///
/// pub trait Visitor {
///     fn visit_arc(&mut self, arc: &Arc) {
///         visit_arc(self, arc)
///     }
///     fn visit_rectangle(&mut self, rectangle: &Rectangle) {
///         visit_rectangle(self, rectangle)
///     }
///     fn visit_point(&mut self, point: &Point) {
///         visit_point(self, point)
///     }
/// }
///
/// pub fn visit_arc<V>(visitor: &mut V, _arc: &Arc)
/// where
///     V: Visitor + ?Sized,
/// {
/// }
/// pub fn visit_rectangle<V>(visitor: &mut V, _rectangle: &Rectangle)
/// where
///     V: Visitor + ?Sized,
/// {
/// }
/// pub fn visit_point<V>(_visitor: &mut V, _point: &Point)
/// where
///     V: Visitor + ?Sized,
/// {
/// }
///
/// trait Visitable {
///     fn apply(&self, visitor: &mut impl Visitor);
/// }
/// impl Visitable for Arc {
///     fn apply(&self, visitor: &mut impl Visitor) {
///         visitor.visit_arc(self);
///     }
/// }
/// impl Visitable for Rectangle {
///     fn apply(&self, visitor: &mut impl Visitor) {
///         visitor.visit_rectangle(self);
///     }
/// }
/// impl Visitable for Point {
///     fn apply(&self, visitor: &mut impl Visitor) {
///         visitor.visit_point(self);
///     }
/// }
///
/// struct Arc {
///    center: Point,
///    radius: u32,
/// }
///
/// struct Rectangle {
///     top_right: Point,
///     bottom_left: Point,
/// }
///
/// struct Point {
///     x: i32,
///     y: i32,
/// }
/// ```
///
/// The input shows `Visitor` can be applied to structs, but the macro also supports traits.
///
/// ## Usage
/// Any visitor can now just implement the `Visitor` trait and provide its own implementation for any of the visitor methods.
/// ```
/// # use despatma::visitor;
/// #
/// # visitor!(
/// #     Arc,
/// #     Rectangle,
/// #     Point,
/// # );
/// #
/// # struct Arc {
/// #    center: Point,
/// #    radius: u32,
/// # }
/// #
/// # struct Rectangle {
/// #     top_right: Point,
/// #     bottom_left: Point,
/// # }
/// #
/// # struct Point {
/// #     x: i32,
/// #     y: i32,
/// # }
/// #
/// struct PointCounter {
///     pub count: usize,
/// }
///
/// impl Visitor for PointCounter {
///     // Only override the visit_point method
///     // All other methods will call the default helper function since we don't care about those type.
///     // But we still need to call the helper function to keep the traversal intact.
///     fn visit_point(&mut self, point: &Point) {
///         self.count += 1;
///
///         // Call helper function to keep traversal intact
///         visit_point(self, point)
///     }
/// }
/// ```
///
/// This visitor will now count all the points in a hierarchy.
/// But there is a problem. The default helper implementations do not have any traversal code. This can be fixed with the `helper_tmpl` option.
///
/// ## `helper_tmpl` option
/// This option will fill the body of the helper method with the given code.
/// ```
/// use despatma::visitor;
/// #
/// # struct Arc {
/// #    center: Point,
/// #    radius: u32,
/// # }
/// #
/// # struct Rectangle {
/// #     top_right: Point,
/// #     bottom_left: Point,
/// # }
/// #
/// # struct Point {
/// #     x: i32,
/// #     y: i32,
/// # }
///
/// visitor!(
///     #[helper_tmpl = {visitor.visit_point(&arc.center);} ]
///     Arc,
///
///     #[
///         helper_tmpl = {
///             visitor.visit_point(&rectangle.top_right);
///             visitor.visit_point(&rectangle.bottom_left);
///         },
///     ]
///     Rectangle,
///
///     Point,
/// );
/// ```
///
/// The helper functions will now look as follow:
/// ```
/// // `Visitor` is same as earlier
/// # pub trait Visitor {
/// #     fn visit_arc(&mut self, arc: &Arc) {
/// #         visit_arc(self, arc)
/// #     }
/// #     fn visit_rectangle(&mut self, rectangle: &Rectangle) {
/// #         visit_rectangle(self, rectangle)
/// #     }
/// #     fn visit_point(&mut self, point: &Point) {
/// #         visit_point(self, point)
/// #     }
/// # }
///
/// pub fn visit_arc<V>(visitor: &mut V, arc: &Arc)
/// where
///     V: Visitor + ?Sized,
/// {
///     visitor.visit_point(&arc.center);
/// }
/// pub fn visit_rectangle<V>(visitor: &mut V, rectangle: &Rectangle)
/// where
///     V: Visitor + ?Sized,
/// {
///     visitor.visit_point(&rectangle.top_right);
///     visitor.visit_point(&rectangle.bottom_left);
/// }
/// pub fn visit_point<V>(_visitor: &mut V, _point: &Point)
/// where
///     V: Visitor + ?Sized,
/// {
/// }
///
/// // `Visitable` is same as earlier
/// # struct Arc {
/// #    center: Point,
/// #    radius: u32,
/// # }
/// #
/// # struct Rectangle {
/// #     top_right: Point,
/// #     bottom_left: Point,
/// # }
/// #
/// # struct Point {
/// #     x: i32,
/// #     y: i32,
/// # }
/// ```
///
/// `PointCounter` will now work as expected!
///
/// ## `no_defualt` option
/// You might want to force visitors to implement a visit method and not have a trait default. The default trait implementation can be removed using the `no_default` option.
/// ```
/// use despatma::visitor;
/// #
/// # struct Arc {
/// #    center: Point,
/// #    radius: u32,
/// # }
/// #
/// # struct Rectangle {
/// #     top_right: Point,
/// #     bottom_left: Point,
/// # }
/// #
/// # struct Point {
/// #     x: i32,
/// #     y: i32,
/// # }
///
/// visitor!(
///     #[
///         helper_tmpl = {visitor.visit_point(&arc.center);},
///         no_default,
///     ]
///     Arc,
///
///     #[
///         helper_tmpl = {
///             visitor.visit_point(&rectangle.top_right);
///             visitor.visit_point(&rectangle.bottom_left);
///         },
///     ]
///     Rectangle,
///
///     Point,
/// );
/// ```
///
/// The `Visitor` trait will now be as follow and `PointCounter` will have to implement the `visit_arc()` method too.
/// ```
/// pub trait Visitor {
///     fn visit_arc(&mut self, arc: &Arc);
///     fn visit_rectangle(&mut self, rectangle: &Rectangle) {
///         visit_rectangle(self, rectangle)
///     }
///     fn visit_point(&mut self, point: &Point) {
///         visit_point(self, point)
///     }
/// }
///
/// // Rest is same as earlier
/// # pub fn visit_arc<V>(visitor: &mut V, arc: &Arc)
/// # where
/// #     V: Visitor + ?Sized,
/// # {
/// #     visitor.visit_point(&arc.center);
/// # }
/// # pub fn visit_rectangle<V>(visitor: &mut V, rectangle: &Rectangle)
/// # where
/// #     V: Visitor + ?Sized,
/// # {
/// #     visitor.visit_point(&rectangle.top_right);
/// #     visitor.visit_point(&rectangle.bottom_left);
/// # }
/// # pub fn visit_point<V>(_visitor: &mut V, _point: &Point)
/// # where
/// #     V: Visitor + ?Sized,
/// # {
/// # }
/// #
/// # struct Arc {
/// #    center: Point,
/// #    radius: u32,
/// # }
/// #
/// # struct Rectangle {
/// #     top_right: Point,
/// #     bottom_left: Point,
/// # }
/// #
/// # struct Point {
/// #     x: i32,
/// #     y: i32,
/// # }
/// ```
///
/// # Calling a visitor
/// Suppose the follow code exists
/// ```
/// # use despatma::visitor;
/// #
/// # visitor!(
/// #     Arc,
/// #     Rectangle,
/// #     Point,
/// # );
/// #
/// # struct Arc {
/// #    center: Point,
/// #    radius: u32,
/// # }
/// #
/// # struct Rectangle {
/// #     top_right: Point,
/// #     bottom_left: Point,
/// # }
/// #
/// # impl Rectangle {
/// #     fn new(left: i32, bottom: i32, right: i32, top: i32) -> Self {
/// #         Self {
/// #             top_right: Point { x: right, y: top },
/// #             bottom_left: Point { x: left, y: bottom },
/// #         }
/// #     }
/// # }
/// #
/// #
/// # struct Point {
/// #     x: i32,
/// #     y: i32,
/// # }
/// #
/// # struct PointCounter {
/// #     pub count: usize,
/// # }
/// #
/// # impl Visitor for PointCounter {
/// #     fn visit_point(&mut self, point: &Point) {
/// #         self.count += 1;
/// #
/// #         // Call helper function to keep traversal intact
/// #         visit_point(self, point)
/// #     }
/// # }
/// // Create a rectangle with bottom-left point (0, 0) and top-right point (10, 12)
/// let rect = Rectangle::new(0, 0, 10, 12);
/// let mut point_stats = PointCounter{ count: 0 };
///
/// // Invoke visitor on hierarchy options
/// rect.apply(&mut point_stats); // 1 - Preferred
/// visit_rectangle(&mut point_stats, &rect); // 2
/// point_stats.visit_rectangle(&rect); // 3
/// ```
///
/// The visitor (`PointCounter`) can be invoked in three ways
/// 1. This is the preferred way as it will work with any visitor and there is no need to remember the visit method's name.
/// 1. Needs to know the helper function name and is less generic. But it might work with any visitor.
/// 1. Least generic and also needs to know the method name.
pub use despatma_visitor::visitor;

/// Like [visitor] but allows the mutation of each item visited
pub use despatma_visitor::visitor_mut;

/// The `dependency_container` macro simplifies dependency injection in Rust by automatically wiring dependencies based on an `impl` block. It creates a dependency container with public methods that handle the correct setup and wiring of dependencies.
///
/// ## Basic Usage
///
/// ```
/// use despatma::dependency_container;
///
/// struct Config {
///     port: u32,
/// }
///
/// struct Service;
///
/// impl Service {
///     pub fn new(port: u32) -> Self {
///         Self
///     }
/// }
///
/// #[dependency_container]
/// impl MyContainer {
///     fn config(&self) -> Config {
///         Config { port: 8080 }
///     }
///
///     fn service(&self, config: Config) -> Service {
///         Service::new(config.port)
///     }
/// }
///
/// let container = MyContainer::new();
/// let service = container.service();
/// ```
///
/// In this example:
/// - The macro creates a `MyContainer` struct based on the name in the `impl` block.
/// - The macro also created a `new()` method to instantiate the container.
/// - Public `config` and `service` methods are generated.
/// - The `service` method is automatically wired to use the `config` method's output.
///
/// **Important**: The linking between dependencies works because the `config()` method has the same name as the `config` argument in the `service` method. This name matching is crucial for the auto-wiring to function correctly.
///
/// ## Container Visibility
///
/// By default, the generated container struct is private. To make it public, add a visibility modifier to the macro invocation:
///
/// ```
/// use despatma::dependency_container;
///
/// #[dependency_container(pub)]
/// impl MyContainer { }
/// ```
///
/// The macro supports all Rust [visibility modifiers](https://doc.rust-lang.org/reference/visibility-and-privacy.html) including `pub`, `pub(crate)`, `pub(super)`, and `pub(in path)`.
///
/// ## Advanced Features
///
/// ### Returning Traits
///
/// The `dependency_container` macro supports returning trait objects. This unlocks a number of benefits:
/// 1. The `Service` can be decoupled from the concrete implementation of `DataLayer`. This makes it easier to write unit
///    tests for the `Service` without needing the production `DataLayer`.
/// 2. By returning a trait object, the `DataLayer` could even be swapped out at runtime. This can be useful for having
///    new features behind some feature flag. If these features then fails in production, then an immediate rollback is
///    possible without needing to do a redeploy.
///
/// ```
/// use despatma::dependency_container;
///
/// trait DataLayer {
///     fn get_user_name(&self, id: u32) -> String;
/// }
///
/// // Implementation details...
/// # struct Sqlite;
/// #
/// # impl DataLayer for Sqlite {
/// #     fn get_user_name(&self, id: u32) -> String {
/// #          format!("User {}", id)
/// #     }
/// # }
/// #
/// # struct Service<D: DataLayer> {
/// #     data_layer: D,
/// # }
/// #
/// # impl<D: DataLayer> Service<D> {
/// #     pub fn new(data_layer: D) -> Self {
/// #         Self { data_layer }
/// #     }
/// # }
///
/// #[dependency_container]
/// impl Dependencies {
///     #[Transient(Sqlite)]
///     fn data_layer(&self) -> impl DataLayer {
///         Sqlite
///     }
///
///     fn service(&self, data_layer: impl DataLayer) -> Service<impl DataLayer> {
///         Service::new(data_layer)
///     }
/// }
/// ```
///
/// The `data_layer` function is now the only thing that ever needs to be modified when changing the concrete
/// implementation of `DataLayer`. Note that the macro does need a hint about the concrete type to use for the trait though.
///
/// ### Runtime Abstractions
///
/// For runtime dependency switching, you can use `Box<dyn Trait>`:
///
/// ```
/// use auto_impl::auto_impl;
/// use despatma::dependency_container;
///
/// #[auto_impl(Box)]
/// trait DataLayer {
///     fn get_user_name(&self, id: u32) -> String;
/// }
///
/// // Implementation details...
/// # struct Config {
/// #     use_sqlite: bool,
/// # }
/// #
/// # struct Sqlite;
/// #
/// # impl DataLayer for Sqlite {
/// #     fn get_user_name(&self, id: u32) -> String {
/// #          format!("Sqlite User {}", id)
/// #     }
/// # }
/// #
/// # struct Postgres;
/// #
/// # impl DataLayer for Postgres {
/// #     fn get_user_name(&self, id: u32) -> String {
/// #         format!("Postgres User {}", id)
/// #     }
/// # }
/// #
/// # struct Service<D: DataLayer> {
/// #     data_layer: D,
/// # }
/// #
/// # impl<D: DataLayer> Service<D> {
/// #     pub fn new(data_layer: D) -> Self {
/// #         Self { data_layer }
/// #     }
/// # }
///
/// #[dependency_container]
/// impl DependencyContainer {
/// #   fn config(&self) -> Config {
/// #       Config { use_sqlite: true }
/// #   }
/// #
///     #[Transient(Box<dyn DataLayer>)]
///     fn data_layer(&self, config: Config) -> impl DataLayer {
///         let dl: Box<dyn DataLayer> = if config.use_sqlite {
///             Box::new(Sqlite)
///         } else {
///             Box::new(Postgres)
///         };
///         dl
///     }
///     // Other methods...
/// }
/// ```
///
/// **Important**: To make this work:
/// 1. Annotate the `DataLayer` trait with `#[auto_impl(Box)]`. This implements the `DataLayer` trait for `Box<dyn DataLayer>`.
/// 2. Use `impl DataLayer` as the return type, but create a `Box<dyn DataLayer>` internally to handle different concrete types.
///
/// ### Async Dependencies
///
/// The macro supports async dependencies by automatically making parent dependencies async:
///
/// ```
/// use despatma::dependency_container;
/// # use std::time::Duration;
/// # use tokio::time::sleep;
///
/// // Implementation details...
/// # struct Config {
/// #     port: u32,
/// # }
/// #
/// # impl Config {
/// #     async fn new() -> Self {
/// #         sleep(Duration::from_secs(1)).await;
/// #         Config { port: 8080 }
/// #     }
/// # }
/// #
/// # struct Service;
/// #
/// # impl Service {
/// #     pub fn new(port: u32) -> Self {
/// #         Service
/// #     }
/// # }
///
/// #[dependency_container]
/// impl MyContainer {
///     async fn config(&self) -> Config {
///         Config::new().await
///     }
///
///     fn service(&self, config: Config) -> Service {
///         Service::new(config.port)
///     }
/// }
/// ```
///
/// Note that the `service` method will be automatically made `async` by the macro to accommodate the async `config` dependency.
///
/// ### Singleton / Scoped Dependencies
/// This macro also allows for the management of singleton or scoped dependencies:
///
/// ```
/// use despatma::dependency_container;
/// # use std::time::Duration;
/// # use tokio::time::sleep;
///
/// // Implementation details...
/// # struct Config {
/// #     port: u32,
/// # }
/// #
/// # impl Config {
/// #     async fn new() -> Self {
/// #         sleep(Duration::from_secs(1)).await;
/// #         Config { port: 8080 }
/// #     }
/// # }
/// #
/// # struct Service;
/// #
/// # impl Service {
/// #     pub fn new(port: u32) -> Self {
/// #         Service
/// #     }
/// # }
///
/// #[dependency_container]
/// impl MyContainer {
///     #[Singleton]
///     async fn config(&self) -> Config {
///         Config::new().await
///     }
///
///     fn service(&self, config: &Config) -> Service {
///         Service::new(config.port)
///     }
/// }
///
/// let container = MyContainer::new();
/// let service = container.service();
///
/// let new_container_scope = container.new_scope();
/// let service2 = container.service();
/// ```
///
/// This is done by adding the `#[Singleton]` or `#[Scoped]` attribute to the registration method of the respective dependency.
/// In this instance, it will cause the `Config` to only be constructed once - when it is requested for the first time.
/// But `Service` will be constructed each time it is requested.
/// New scopes can be started by calling the `new_scope` method on the container.
///
/// **Important**: To make this work:
/// 1. The `service` method needs to take a reference to the `Config` object since the config is now a singleton. Ie we
///    only want one instance of it to exist, so can't have multiple owned instances of it floating around.
///
/// The following dependency lifetimes are supported:
/// - `#[Singleton]`: The dependency is created once and shared across all requests.
/// - `#[Scoped]`: The dependency is created once per scope and shared across all requests within that scope.
/// - `#[Transient]`: The dependency is created each time it is requested. This is the default when no attribute is
///   provided.
///
///
/// #### With Abstractions
/// Sometimes you might want to use a trait object for the singleton or scoped dependency. In these instances the container
/// will need to know a concrete type to store the singleton or scoped dependency.
/// This can be done by hinting the type on the `Singleton` or `Scoped` attribute:
///
/// ```
/// use auto_impl::auto_impl;
/// use despatma::dependency_container;
///
/// #[auto_impl(&)]
/// trait DataLayer {
///     fn get_user_name(&self, id: u32) -> String;
/// }
///
/// // Implementation details...
/// # struct Sqlite;
/// #
/// # impl DataLayer for Sqlite {
/// #     fn get_user_name(&self, id: u32) -> String {
/// #          format!("User {}", id)
/// #     }
/// # }
/// #
/// # struct Service<D: DataLayer> {
/// #     data_layer: D,
/// # }
/// #
/// # impl<D: DataLayer> Service<D> {
/// #     pub fn new(data_layer: D) -> Self {
/// #         Self { data_layer }
/// #     }
/// # }
///
/// #[dependency_container]
/// impl Dependencies {
///     #[Singleton(Sqlite)]
///     fn data_layer(&self) -> impl DataLayer {
///         Sqlite
///     }
///
///     fn service(&self, data_layer: impl DataLayer) -> Service<impl DataLayer> {
///         Service::new(data_layer)
///     }
/// }
/// ```
///
/// **Important**: To make this work:
/// 1. Annotate the `DataLayer` trait with `#[auto_impl(&)]`. This implements the `DataLayer` trait for references to it
///    too. We need this since we are still only giving a reference to `service` when it requests the `DataLayer`
///    dependency. However, `service` no longer needs to know it is getting a reference like the previous example.
///
/// ### Constructor arguments
///
/// In some cases, you may need to initialize dependencies outside the container. In such cases, you can define a static `new` method with arguments listing dependencies of this type.
///
/// **Important**: The argument names must match those in the location where these dependencies are used.
///
/// ```
/// use despatma::dependency_container;
///
/// struct Config {
///     port: u32,
/// }
///
/// struct Service;
///
/// impl Service {
///     fn new(port: u32) -> Self {
///         println!("Service started on port {}", port);
///         Self
///     }
/// }
///
/// #[dependency_container]
/// impl DependencyContainer {
///     fn new(config: Config) {}
///
///     fn service(&self, config: &Config) -> Service {
///         Service::new(config.port)
///     }
/// }
///
/// let config = Config { port: 8080 };
/// let container = DependencyContainer::new(config);
/// let _service = container.service();
/// ```
///
/// ## Considerations
///
/// - The macro determines wiring based on method names matching argument names.
/// - When using runtime abstractions, ensure you're following the pattern shown in the `Box<dyn Trait>` example.
/// - Async dependencies will cause parent dependencies to become async as well.
/// - Consider the performance implications of excessive boxing or async calls in your dependency tree.
///
/// More [advanced example are also in the repository](https://github.com/chesedo/despatma/tree/main/despatma/examples).
/// For more information on dependency injection in Rust, see this article on [Manual Dependency Injection in Rust](https://chesedo.me/blog/manual-dependency-injection-rust/).
pub use despatma_dependency_container::dependency_container;

// Re-export this since it is used by the dependency_container macro
pub use async_once_cell;
