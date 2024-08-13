use std::fs;

#[test]
pub fn pass_abstract_factory() {
    macrotest::expand("tests/expand/abstract_factory/*.rs");
}

#[test]
pub fn pass_dependency_container() {
    let t = trybuild::TestCases::new();

    // Use glob to get all .rs files that don't end with .expanded.rs
    // These files are used by macrotest
    let pattern = "tests/expand/dependency_container";
    for path in fs::read_dir(pattern)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|name| {
            !name
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .ends_with(".expanded.rs")
        })
    {
        t.pass(path);
    }

    t.pass("tests/expand/dependency_container/*[!.expanded].rs");
    // Get any errors before we run the macrotest
    // Else macrotest might fail and we won't know why
    drop(t);

    // macrotest::expand("tests/expand/dependency_container/*.rs");
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
