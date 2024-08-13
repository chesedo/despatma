#[test]
pub fn pass_abstract_factory() {
    macrotest::expand("tests/expand/abstract_factory/*.rs");
}

#[test]
pub fn pass_dependency_container() {
    macrotest::expand("tests/expand/dependency_container/*.rs");
}

#[test]
pub fn pass_visitor() {
    macrotest::expand("tests/expand/visitor/*.rs");
}

#[test]
pub fn fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fail/abstract_factory/*.rs");
    t.compile_fail("tests/fail/visitor/*.rs");
}
