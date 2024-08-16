use std::fs;

#[test]
pub fn expand() {
    let t = trybuild::TestCases::new();

    // Use glob to get all .rs files that don't end with .expanded.rs
    // These files are used by macrotest
    let pattern = "tests/expand";
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

    // Get any errors before we run the macrotest
    // Else macrotest might fail and we won't know why
    drop(t);

    macrotest::expand("tests/expand/*.rs");
}

#[test]
pub fn fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fail/*.rs");
}
