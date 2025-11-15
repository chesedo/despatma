//! Test all the visibility levels of generated dependency containers.

#[despatma_dependency_container::dependency_container]
impl PrivateDependencyContainer {}

#[despatma_dependency_container::dependency_container(pub)]
impl PublicDependencyContainer {}

#[despatma_dependency_container::dependency_container(pub(crate))]
impl PublicCrateDependencyContainer {}

#[despatma_dependency_container::dependency_container(pub(self))]
impl PublicSelfDependencyContainer {}

mod outer {
    #[despatma_dependency_container::dependency_container(pub(super))]
    impl PublicSuperDependencyContainer {}

    mod inner {
        #[despatma_dependency_container::dependency_container(pub(in crate::outer))]
        impl PublicModInOuterDependencyContainer {}
    }
}

fn main() {}
