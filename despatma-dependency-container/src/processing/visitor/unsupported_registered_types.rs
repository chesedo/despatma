use proc_macro_error2::emit_error;
use syn::{FnArg, Pat};

use crate::processing::Dependency;

use super::{ErrorVisitorMut, VisitorMut};

/// Reports on any requested dependencies which are of an unsupported type.
pub struct UnsupportedRegisteredTypes {
    types: Vec<Pat>,
}

impl VisitorMut for UnsupportedRegisteredTypes {
    fn visit_dependency_mut(&mut self, dependency: &mut Dependency) {
        let unsupported = dependency
            .sig
            .inputs
            .iter()
            .filter_map(|fn_arg| match fn_arg {
                FnArg::Receiver(_) => None,
                FnArg::Typed(pat_type) => Some(pat_type),
            })
            .filter_map(|pat_type| match &*pat_type.pat {
                Pat::Ident(_) => None,
                pat => Some(pat),
            })
            .cloned();

        self.types.extend(unsupported);
    }
}

impl ErrorVisitorMut for UnsupportedRegisteredTypes {
    fn new() -> Self {
        Self {
            types: Default::default(),
        }
    }

    fn emit_errors(self) {
        for pat in self.types {
            emit_error!(pat, "This argument type is not supported");
        }
    }
}
