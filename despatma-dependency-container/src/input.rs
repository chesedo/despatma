use proc_macro_error2::emit_error;
use syn::{Attribute, ImplItem, ImplItemFn, ItemImpl, Type, Visibility};

#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub struct Container {
    pub(crate) vis: Visibility,
    pub(crate) attrs: Vec<Attribute>,
    pub(crate) self_ty: Type,
    pub(crate) dependencies: Vec<ImplItemFn>,
}

impl Container {
    pub fn from_item_impl(item_impl: ItemImpl) -> Self {
        let dependencies = item_impl
            .items
            .into_iter()
            .filter_map(|impl_item| match impl_item {
                ImplItem::Fn(impl_item_fn) => Some(impl_item_fn),
                impl_item => {
                    emit_error!(impl_item, "This impl item is not supported");
                    None
                }
            })
            .collect();

        Self {
            vis: Visibility::Inherited,
            attrs: item_impl.attrs,
            self_ty: item_impl.self_ty.as_ref().clone(),
            dependencies,
        }
    }

    pub fn set_visibility(&mut self, vis: Visibility) {
        self.vis = vis;
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn from_item_impl() {
        let container = Container::from_item_impl(parse_quote!(
            impl DependencyContainer {
                fn config(&self) -> Config {
                    Config
                }

                fn service(&self, config: Config) -> Service {
                    Service::new(config)
                }

                fn service_ref_config(&self, config: &Config) -> Service {
                    Service::new_ref(config)
                }

                async fn async_service(&self, config: Config) -> Service {
                    Service::async_new(config).await
                }

                #[Singleton]
                fn singleton(&self) -> Singleton {
                    Singleton
                }

                #[Scoped]
                fn scoped(&self) -> Scoped {
                    Scoped
                }

                #[Transient]
                fn transient(&self) -> Transient {
                    Transient
                }

                fn default(&self) -> Default {
                    Default
                }
            }
        ));
        let expected = Container {
            vis: Visibility::Inherited,
            attrs: vec![],
            self_ty: parse_quote!(DependencyContainer),
            dependencies: vec![
                parse_quote!(
                    fn config(&self) -> Config {
                        Config
                    }
                ),
                parse_quote!(
                    fn service(&self, config: Config) -> Service {
                        Service::new(config)
                    }
                ),
                parse_quote!(
                    fn service_ref_config(&self, config: &Config) -> Service {
                        Service::new_ref(config)
                    }
                ),
                parse_quote!(
                    async fn async_service(&self, config: Config) -> Service {
                        Service::async_new(config).await
                    }
                ),
                parse_quote!(
                    #[Singleton]
                    fn singleton(&self) -> Singleton {
                        Singleton
                    }
                ),
                parse_quote!(
                    #[Scoped]
                    fn scoped(&self) -> Scoped {
                        Scoped
                    }
                ),
                parse_quote!(
                    #[Transient]
                    fn transient(&self) -> Transient {
                        Transient
                    }
                ),
                parse_quote!(
                    fn default(&self) -> Default {
                        Default
                    }
                ),
            ],
        };

        assert_eq!(container, expected);
    }
}
