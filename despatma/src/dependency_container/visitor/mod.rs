use super::{ChildDependency, Container, Dependency};

mod wiring_visitor;

pub use wiring_visitor::WiringVisitor;

/// A visitor used to validate the struct that will be turned into a dependency container.
/// If the visitor found any errors then they should be emit in [Drop].
#[allow(drop_bounds)]
pub trait MutVisitor: Drop {
    /// Visit the top level container which will be turned into a struct
    fn visit_container(&mut self, container: &Container) {
        mut_visit_container(self, container);
    }

    /// Visit a dependency that was registered
    fn visit_dependency(&mut self, dependency: &Dependency) {
        mut_visit_dependency(self, dependency);
    }

    /// Visit a dependency requested by a registered dependency
    fn visit_child_dependency(&mut self, child_dependency: &ChildDependency) {
        mut_visit_child_dependency(self, child_dependency);
    }
}

fn mut_visit_container<V: MutVisitor + ?Sized>(visitor: &mut V, container: &Container) {
    for dependency in container.dependencies.values() {
        visitor.visit_dependency(dependency);
    }
}

fn mut_visit_dependency<V: MutVisitor + ?Sized>(visitor: &mut V, dependency: &Dependency) {
    for child_dependency in &dependency.dependencies {
        visitor.visit_child_dependency(child_dependency);
    }
}

fn mut_visit_child_dependency<V: MutVisitor + ?Sized>(
    _visitor: &mut V,
    _child_dependency: &ChildDependency,
) {
}
