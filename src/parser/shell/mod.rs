use super::{newline, tab, line_feed, space};
use crate::parser::PathLike;

const DEFAULT_SHELL_PATH: &str = "/bin/bash";

named!(pub shell<Shell>,
    do_parse!(
        tag!("@") >>
        tag!("shell") >>
        tag!(":") >>
        newline >>
        tab >>
        tag!("path") >>
        tag!(":") >>
        space >>
        path: terminated!(Shell::parse_path, line_feed)>> (
            Shell {
                path
            }
        )
    )
);

pub struct Shell<'a> {
    path: &'a [u8],
}

impl Default for Shell<'_> {
    fn default() -> Self {
        Shell {
            path: DEFAULT_SHELL_PATH.as_bytes()
        }
    }
}

impl PathLike for Shell<'_> {}

#[cfg(test)]
impl PartialEq<&str> for Shell<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.path == other.as_bytes()
    }
}

#[cfg(test)]
impl std::fmt::Debug for Shell<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Shell {{ path: {} }}", String::from_utf8_lossy(self.path))
    }
}

#[cfg(test)]
mod tests {
    use super::shell;
    use crate::parser::common::error_fmt;

    #[test]
    fn test_shell_path() {
        let input = indoc::indoc! {"
            @shell:
                path: /bin/bash
        "};

        let result = shell(input.as_bytes());
        assert!(result.is_ok(), format!("Error: {:?}", result.err().map(error_fmt)));
        let (_, shell) = result.unwrap();
        assert_eq!(shell.path, b"/bin/bash");
    }

    #[test]
    fn test_not_shell_target() {
        let input = indoc::indoc! {"
            @not-shell:
                path: /bin/bash
        "};

        let result = shell(input.as_bytes());
        assert!(result.is_err());
    }
}
