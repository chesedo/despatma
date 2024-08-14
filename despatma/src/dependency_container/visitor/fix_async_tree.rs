use std::{cell::RefCell, collections::HashSet, rc::Rc};

use indexmap::IndexMap;
use syn::Ident;

use crate::dependency_container::Dependency;

use super::VisitMut;

/// Visitor used to determine if any child dependencies in the calltree is async. Because if any
/// child dependencies are async, then the parent dependency must also be async. Which this
/// visitor updates correcly.
pub struct FixAsyncTree {
    dependencies: IndexMap<Ident, Rc<RefCell<Dependency>>>,
    visited_dependencies: HashSet<Ident>,
}

impl FixAsyncTree {
    pub fn new(dependencies: IndexMap<Ident, Rc<RefCell<Dependency>>>) -> Self {
        Self {
            dependencies,
            visited_dependencies: HashSet::new(),
        }
    }
}

impl VisitMut for FixAsyncTree {
    fn visit_dependency_mut(&mut self, dependency: &mut Dependency) {
        // We want to be efficient and not visit the same dependency multiple times
        if self.visited_dependencies.contains(&dependency.sig.ident) {
            return;
        }

        let dependencies: Vec<_> = dependency
            .dependencies
            .iter()
            .filter_map(|d| self.dependencies.get(&d.ident))
            .cloned()
            .collect();

        let has_sync_child = dependencies.iter().any(|d| {
            let mut d = d.borrow_mut();

            // Make sure we update any child dependencies first as they might be in an async calltree.
            // Basically the idea is to visit the calltree in a depth-first manner.
            self.visit_dependency_mut(&mut d);

            d.is_async
        });

        dependency.is_async |= has_sync_child;

        self.visited_dependencies
            .insert(dependency.sig.ident.clone());
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use crate::dependency_container::Container;

    use super::*;

    #[test]
    fn fix_async_tree() {
        let mut container = Container::from_item_impl(parse_quote!(
            impl Dependencies {
                fn config(&self) -> Config {
                    Config
                }

                fn service(&self, config: Config, db: Db) -> Service {
                    Service(config, db)
                }

                async fn logger(&self) -> Logger {
                    Logger
                }

                fn db(&self, logger: Logger) -> Db {
                    Db
                }
            }
        ));

        let mut async_visitor = FixAsyncTree::new(container.dependencies.clone());
        async_visitor.visit_container_mut(&mut container);

        assert!(
            !container
                .dependencies
                .get::<Ident>(&parse_quote!(config))
                .unwrap()
                .borrow()
                .is_async
        );
        assert!(
            container
                .dependencies
                .get::<Ident>(&parse_quote!(logger))
                .unwrap()
                .borrow()
                .is_async
        );
        assert!(
            container
                .dependencies
                .get::<Ident>(&parse_quote!(db))
                .unwrap()
                .borrow()
                .is_async
        );
        assert!(
            container
                .dependencies
                .get::<Ident>(&parse_quote!(service))
                .unwrap()
                .borrow()
                .is_async
        );
    }
}
