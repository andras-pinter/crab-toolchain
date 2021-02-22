mod manifest;
mod arguments;
mod name;

use crate::parser::{space, newline, tab, digit, line_feed};
use manifest::{manifest, Manifest};
use name::container_name;
use arguments::{argument, Argument};

named!(pub container<Container>,
    do_parse!(
            tag!("@") >>
            name: terminated!(container_name, newline) >>
            manifest: preceded!(tab, manifest) >>
            arguments: verify!(many0!(complete!(preceded!(tab, argument))), Container::verify_arguments) >>
            alt!(newline | eof!()) >> (
                Container {
                    name,
                    manifest,
                    arguments
                }
            )
        )
);

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Container<'a> {
    name: &'a [u8],
    manifest: Manifest<'a>,
    arguments: Vec<Argument<'a>>
}

impl<'a> Container<'a> {
    fn verify_arguments(arguments: &[Argument<'a>]) -> bool {
        let mut unique = std::collections::HashSet::new();
        arguments.iter().all(move |arg| unique.insert(arg))
    }
}

impl<'a> std::iter::FromIterator<Container<'a>> for std::collections::HashMap<&'a [u8], Container<'a>> {
    fn from_iter<T: IntoIterator<Item=Container<'a>>>(iter: T) -> Self {
        iter.into_iter().map(|c| (c.name, c)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{container, Manifest, Argument};
    use crate::parser::common::error_fmt;

    #[test]
    fn test_parse_simple_input() {
        let input = indoc::indoc! {"
            @ubuntu:
                from: ubuntu:latest
        "};

        let result = container(input.as_bytes());
        assert!(result.is_ok(), format!("Error: {:?}", result.err().map(error_fmt)));
        let (remaining, container) = result.unwrap();
        assert!(remaining.is_empty(), format!("Remaining input should be empty: {}", String::from_utf8_lossy(remaining)));
        assert_eq!(container.name, b"ubuntu");
        assert_eq!(container.manifest, Manifest::Image(b"ubuntu:latest"))
    }

    #[test]
    fn test_parse_input_with_arguments() {
        let input = indoc::indoc! {"
            @ubuntu:
                from: ubuntu:latest
                port: 80:8080
                volume: /usr/lib/:/usr/share/lib
                volume-from: cache_container
                expose: 443
                volume: /home/apple:/home/peach
        "};

        let result = container(input.as_bytes());
        assert!(result.is_ok(), format!("Error: {:?}", result.err().map(error_fmt)));
        let (remaining, container) = result.unwrap();
        assert!(remaining.is_empty(), format!("Remaining input should be empty: {}", String::from_utf8_lossy(remaining)));
        assert_eq!(container.name, b"ubuntu");
        assert_eq!(container.manifest, Manifest::Image(b"ubuntu:latest"));
        assert_eq!(container.arguments, vec![
            Argument::PublishPort {
                outer: 80,
                inner: 8080
            },
            Argument::Volume {
                source: b"/usr/lib/",
                mount: b"/usr/share/lib"
            },
            Argument::VolumeFrom {
                name: b"cache_container"
            },
            Argument::ExposePort {
                port: 443
            },
            Argument::Volume {
                source: b"/home/apple",
                mount: b"/home/peach"
            }
        ])
    }

    #[test]
    fn test_parse_input_with_multiple_same_arguments() {
        let input = indoc::indoc! {"
            @ubuntu:
                from: ubuntu:latest
                port: 80:8080
                port: 443:8443
        "};

        let result = container(input.as_bytes());
        assert!(result.is_ok(), format!("Error: {:?}", result.err().map(error_fmt)));
        let (remaining, container) = result.unwrap();
        assert!(remaining.is_empty(), format!("Remaining input should be empty: {}", String::from_utf8_lossy(remaining)));
        assert_eq!(container.name, b"ubuntu");
        assert_eq!(container.manifest, Manifest::Image(b"ubuntu:latest"));
        assert_eq!(container.arguments, vec![
            Argument::PublishPort {
                outer: 80,
                inner: 8080
            },
            Argument::PublishPort {
                outer: 443,
                inner: 8443
            },
        ])
    }

    #[test]
    fn test_parse_input_with_duplicated_manifest() {
        let input = indoc::indoc! {"
            @ubuntu:
                from: ubuntu:latest
                from: ubuntu:bionic
                port: 80:8080
        "};

        let result = container(input.as_bytes());
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_input_with_duplicated_argument() {
        let input = indoc::indoc! {"
            @ubuntu:
                from: ubuntu:latest
                port: 80:8080
                port: 80:8080
        "};

        let result = container(input.as_bytes());
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_argument() {
        let input = indoc::indoc! {"
            @ubuntu:
                from: ubuntu:latest
                invalid: invalid
                port: 80:8080
        "};

        let result = container(input.as_bytes());
        assert!(result.is_err());
    }
}
