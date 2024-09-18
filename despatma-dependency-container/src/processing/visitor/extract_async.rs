use crate::processing::Dependency;

use super::{ErrorVisitorMut, VisitorMut};

/// Fix up any dependencies that might be async. It checks whether the registered function is async. Or whether any
/// child dependencies are async. Because if any child dependencies are async, then the parent dependency must also be
/// async. Which this visitor updates correcly.
///
/// Requires child dependencies to be linked first.
pub struct ExtractAsync;

impl VisitorMut for ExtractAsync {
    fn visit_dependency_mut(&mut self, dependency: &mut Dependency) {
        dependency.is_async = dependency.sig.asyncness.is_some();

        for child in dependency.dependencies.iter_mut() {
            let child = &child.inner;
            self.visit_dependency_mut(&mut child.borrow_mut());

            dependency.is_async |= child.borrow().is_async;
        }
    }
}

impl ErrorVisitorMut for ExtractAsync {
    fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use crate::{
        input,
        processing::{
            self,
            visitor::{ErrorVisitorMut, LinkDependencies, VisitableMut},
        },
    };

    use super::*;

    #[test]
    fn extract_async() {
        let mut container: processing::Container = input::Container::from_item_impl(parse_quote!(
            impl Container {
                // Have this first to make sure child dependencies are considered correctly
                fn service(&self, middle_foo: u32) -> u32 {
                    async_foo
                }

                fn foo(&self) -> u32 {
                    42
                }

                async fn async_foo(&self) -> u32 {
                    42
                }

                fn middle_foo(&self, async_foo: u32) -> u32 {
                    async_foo
                }
            }
        ))
        .into();

        // Test needs them to be linked
        container.apply_mut(&mut LinkDependencies::new());

        assert!(!container.dependencies[0].borrow().is_async);
        assert!(!container.dependencies[1].borrow().is_async);
        assert!(!container.dependencies[2].borrow().is_async);
        assert!(!container.dependencies[3].borrow().is_async);

        container.apply_mut(&mut ExtractAsync);

        assert!(container.dependencies[0].borrow().is_async);
        assert!(!container.dependencies[1].borrow().is_async);
        assert!(container.dependencies[2].borrow().is_async);
        assert!(container.dependencies[3].borrow().is_async);
    }
}
