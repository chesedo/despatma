use super::{ChildDependency, Container, Dependency};

mod async_visitor;
mod impl_trait_but_registered_concrete;
mod wiring_visitor;

pub use async_visitor::AsyncVisitor;
pub use impl_trait_but_registered_concrete::ImplTraitButRegisteredConcrete;
pub use wiring_visitor::WiringVisitor;

/// A visitor used to validate the struct that will be turned into a dependency container.
/// If the visitor found any errors then they should be emit in [emit_errors].
pub trait Visit {
    /// Visit the top level container which will be turned into a struct
    fn visit_container(&mut self, container: &Container) {
        visit_container(self, container);
    }

    /// Visit a dependency that was registered
    fn visit_dependency(&mut self, dependency: &Dependency) {
        visit_dependency(self, dependency);
    }

    /// Visit a dependency requested by a registered dependency
    fn visit_child_dependency(&mut self, child_dependency: &ChildDependency) {
        visit_child_dependency(self, child_dependency);
    }

    /// Emit any errors that were found during the visit
    fn emit_errors(self);
}

fn visit_container<V: Visit + ?Sized>(visitor: &mut V, container: &Container) {
    for dependency in container.dependencies.values() {
        visitor.visit_dependency(&dependency.borrow());
    }
}

fn visit_dependency<V: Visit + ?Sized>(visitor: &mut V, dependency: &Dependency) {
    for child_dependency in &dependency.dependencies {
        visitor.visit_child_dependency(child_dependency);
    }
}

fn visit_child_dependency<V: Visit + ?Sized>(
    _visitor: &mut V,
    _child_dependency: &ChildDependency,
) {
}

/// A mutable visitor used to update any dependencies or their children
pub trait VisitMut {
    fn visit_container_mut(&mut self, container: &mut Container) {
        visit_container_mut(self, container);
    }

    fn visit_dependency_mut(&mut self, dependency: &mut Dependency) {
        visit_dependency_mut(self, dependency);
    }

    fn visit_child_dependency_mut(&mut self, child_dependency: &mut ChildDependency) {
        visit_child_dependency_mut(self, child_dependency);
    }
}

fn visit_container_mut<V: VisitMut + ?Sized>(visitor: &mut V, container: &mut Container) {
    for dependency in container.dependencies.values_mut() {
        visitor.visit_dependency_mut(&mut dependency.borrow_mut());
    }
}

fn visit_dependency_mut<V: VisitMut + ?Sized>(visitor: &mut V, dependency: &mut Dependency) {
    for child_dependency in &mut dependency.dependencies {
        visitor.visit_child_dependency_mut(child_dependency);
    }
}

fn visit_child_dependency_mut<V: VisitMut + ?Sized>(
    _visitor: &mut V,
    _child_dependency: &mut ChildDependency,
) {
}
