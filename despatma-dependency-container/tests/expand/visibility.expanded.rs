//! Test all the visibility levels of generated dependency containers.
struct PrivateDependencyContainer<'a> {
    _phantom: std::marker::PhantomData<&'a ()>,
}
#[automatically_derived]
impl<'a> ::core::clone::Clone for PrivateDependencyContainer<'a> {
    #[inline]
    fn clone(&self) -> PrivateDependencyContainer<'a> {
        PrivateDependencyContainer {
            _phantom: ::core::clone::Clone::clone(&self._phantom),
        }
    }
}
impl<'a> PrivateDependencyContainer<'a> {
    pub fn new() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
    pub fn new_scope(&self) -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}
pub struct PublicDependencyContainer<'a> {
    _phantom: std::marker::PhantomData<&'a ()>,
}
#[automatically_derived]
impl<'a> ::core::clone::Clone for PublicDependencyContainer<'a> {
    #[inline]
    fn clone(&self) -> PublicDependencyContainer<'a> {
        PublicDependencyContainer {
            _phantom: ::core::clone::Clone::clone(&self._phantom),
        }
    }
}
impl<'a> PublicDependencyContainer<'a> {
    pub fn new() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
    pub fn new_scope(&self) -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}
pub(crate) struct PublicCrateDependencyContainer<'a> {
    _phantom: std::marker::PhantomData<&'a ()>,
}
#[automatically_derived]
impl<'a> ::core::clone::Clone for PublicCrateDependencyContainer<'a> {
    #[inline]
    fn clone(&self) -> PublicCrateDependencyContainer<'a> {
        PublicCrateDependencyContainer {
            _phantom: ::core::clone::Clone::clone(&self._phantom),
        }
    }
}
impl<'a> PublicCrateDependencyContainer<'a> {
    pub fn new() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
    pub fn new_scope(&self) -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}
pub(self) struct PublicSelfDependencyContainer<'a> {
    _phantom: std::marker::PhantomData<&'a ()>,
}
#[automatically_derived]
impl<'a> ::core::clone::Clone for PublicSelfDependencyContainer<'a> {
    #[inline]
    fn clone(&self) -> PublicSelfDependencyContainer<'a> {
        PublicSelfDependencyContainer {
            _phantom: ::core::clone::Clone::clone(&self._phantom),
        }
    }
}
impl<'a> PublicSelfDependencyContainer<'a> {
    pub fn new() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
    pub fn new_scope(&self) -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}
mod outer {
    pub(super) struct PublicSuperDependencyContainer<'a> {
        _phantom: std::marker::PhantomData<&'a ()>,
    }
    #[automatically_derived]
    impl<'a> ::core::clone::Clone for PublicSuperDependencyContainer<'a> {
        #[inline]
        fn clone(&self) -> PublicSuperDependencyContainer<'a> {
            PublicSuperDependencyContainer {
                _phantom: ::core::clone::Clone::clone(&self._phantom),
            }
        }
    }
    impl<'a> PublicSuperDependencyContainer<'a> {
        pub fn new() -> Self {
            Self {
                _phantom: Default::default(),
            }
        }
        pub fn new_scope(&self) -> Self {
            Self {
                _phantom: Default::default(),
            }
        }
    }
    mod inner {
        pub(in crate::outer) struct PublicModInOuterDependencyContainer<'a> {
            _phantom: std::marker::PhantomData<&'a ()>,
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for PublicModInOuterDependencyContainer<'a> {
            #[inline]
            fn clone(&self) -> PublicModInOuterDependencyContainer<'a> {
                PublicModInOuterDependencyContainer {
                    _phantom: ::core::clone::Clone::clone(&self._phantom),
                }
            }
        }
        impl<'a> PublicModInOuterDependencyContainer<'a> {
            pub fn new() -> Self {
                Self {
                    _phantom: Default::default(),
                }
            }
            pub fn new_scope(&self) -> Self {
                Self {
                    _phantom: Default::default(),
                }
            }
        }
    }
}
fn main() {}
