#[test]
pub fn expand() {
    macrotest::expand("tests/expand/*.rs");
}

#[test]
pub fn fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fail/*.rs");
}
