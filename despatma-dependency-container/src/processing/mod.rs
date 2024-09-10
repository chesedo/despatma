use std::{cell::RefCell, rc::Rc};

use syn::{parse_quote, Attribute, Block, ImplItemFn, ReturnType, Signature, Type};

use crate::input;

use self::visitor::{
    ErrorVisitorMut, ExtractAsync, ExtractBoxType, ExtractLifetime, ImplTraitButRegisteredConcrete,
    LinkDependencies, UnsupportedRegisteredTypes, VisitableMut,
};

mod visitor;

#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub struct Container {
    pub(crate) attrs: Vec<Attribute>,
    pub(crate) self_ty: Type,
    pub(crate) dependencies: Vec<Rc<RefCell<Dependency>>>,
}

#[derive(Clone)]
#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub struct Dependency {
    pub(crate) attrs: Vec<Attribute>,
    pub(crate) sig: Signature,
    pub(crate) block: Block,
    pub(crate) is_async: bool,
    pub(crate) is_boxed: bool,
    pub(crate) lifetime: Lifetime,
    pub(crate) ty: Type,
    pub(crate) dependencies: Vec<ChildDependency>,
}

#[derive(Clone)]
#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub struct ChildDependency {
    pub(crate) inner: Rc<RefCell<Dependency>>,
    pub(crate) is_ref: bool,
}

#[derive(Clone)]
#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub enum Lifetime {
    Transient,
    Scoped,
    Singleton,
}

impl From<input::Container> for Container {
    fn from(input: input::Container) -> Self {
        let input::Container {
            attrs,
            self_ty,
            dependencies,
        } = input;

        let dependencies = dependencies
            .into_iter()
            .map(Dependency::from)
            .map(RefCell::from)
            .map(Rc::new)
            .collect();

        Self {
            attrs,
            self_ty,
            dependencies,
        }
    }
}

impl From<ImplItemFn> for Dependency {
    fn from(impl_item_fn: ImplItemFn) -> Self {
        let ImplItemFn {
            attrs,
            vis: _,
            defaultness: _,
            sig,
            block,
        } = impl_item_fn;

        let ty = match &sig.output {
            ReturnType::Type(_, ty) => ty.as_ref().clone(),
            ReturnType::Default => parse_quote! { () },
        };

        Self {
            attrs,
            sig,
            block,
            is_async: false,
            is_boxed: false,
            lifetime: Lifetime::Transient,
            ty,
            dependencies: vec![],
        }
    }
}

impl Container {
    pub fn process(&mut self) {
        self.process_visitor::<ExtractLifetime>();

        // Needs lifetimes to be extracted first
        self.process_visitor::<LinkDependencies>();

        // Needs dependencies to be linked first
        // But types should not be changed yet
        self.process_visitor::<ImplTraitButRegisteredConcrete>();

        // Needs dependencies to be linked first
        self.process_visitor::<ExtractAsync>();

        self.process_visitor::<ExtractBoxType>();
        self.process_visitor::<UnsupportedRegisteredTypes>();
    }

    fn process_visitor<V: ErrorVisitorMut>(&mut self) {
        let mut visitor = V::new();

        self.apply_mut(&mut visitor);

        visitor.emit_errors();
    }
}