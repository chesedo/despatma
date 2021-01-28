use std::{
    fmt::Display,
    io::Write,
    process::{Command, Stdio},
};

/// Formats a [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html) using `rustfmt`
/// # Example
/// ```
/// use despatma_test_helpers::reformat;
///
/// assert_eq!(
///     reformat(&String::from("use std::{str,fmt};")),
///     "use std::{fmt, str};\n"
/// )
/// ```
pub fn reformat(text: &dyn Display) -> String {
    let mut rustfmt = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to create command");
    {
        let stdin = rustfmt
            .stdin
            .as_mut()
            .expect("Failed to create input stream");
        stdin
            .write_all(text.to_string().as_bytes())
            .expect("Failed to write to input stream");
    }
    let output = rustfmt
        .wait_with_output()
        .expect("Format command did not end");
    String::from_utf8(output.stdout).expect("Failed to convert output to string")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn reformat_simple() {
        assert_eq!(
            reformat(&String::from("trait Test< SomeType > { }")),
            "trait Test<SomeType> {}\n"
        );
    }
}
