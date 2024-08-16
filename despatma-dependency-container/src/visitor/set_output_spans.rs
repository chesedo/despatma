use std::{cell::RefCell, rc::Rc};

use indexmap::IndexMap;
use syn::{spanned::Spanned, Ident};

use crate::container::{ChildDependency, Dependency};

use super::VisitorMut;

/// Correctly sets the output spans of [ChildDependency]
pub struct SetOutputSpans {
    dependencies: IndexMap<Ident, Rc<RefCell<Dependency>>>,
}

impl SetOutputSpans {
    pub fn new(dependencies: IndexMap<Ident, Rc<RefCell<Dependency>>>) -> Self {
        Self { dependencies }
    }
}

impl VisitorMut for SetOutputSpans {
    fn visit_child_dependency_mut(&mut self, child_dependency: &mut ChildDependency) {
        let Some(dependency) = self.dependencies.get(&child_dependency.ident) else {
            return;
        };

        let dep_ref = dependency.borrow();

        let span = match &dep_ref.sig.output {
            syn::ReturnType::Default => dep_ref.sig.ident.span(),
            syn::ReturnType::Type(_, ty) => ty.span(),
        };

        child_dependency.registered_ty_span = span;
    }
}

#[cfg(test)]
mod tests {
    // Cannot test as [Span] does not impl Eq || PartialEq
}
