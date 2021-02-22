use super::{space, line_feed};

named!(pub manifest<Manifest>,
    do_parse!(
        tag!("from") >>
        tag!(":") >>
        space >>
        manifest: terminated!(Manifest::parse_manifest, line_feed) >> (
            if manifest.starts_with(b"Dockerfile") {
                Manifest::File(manifest)
            } else {
                Manifest::Image(manifest)
            }
        )
    )
);

#[cfg_attr(test, derive(PartialEq))]
pub enum Manifest<'a> {
    File(&'a [u8]),
    Image(&'a [u8]),
}

impl<'a> Manifest<'a> {
    pub fn parse_manifest<T, E: nom::error::ParseError<T>>(input: T) -> nom::IResult<T, T, E>
        where T: nom::InputTakeAtPosition,
              <T as nom::InputTakeAtPosition>::Item: nom::AsChar,
    {
        use nom::AsChar;

        input.split_at_position1_complete(
            |item| match item.as_char() {
                ':' | '_' | '/' | '.' => false,
                chr if chr.is_alphanumeric() => false,
                _ => true
            },
            nom::error::ErrorKind::AlphaNumeric
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{manifest, Manifest};
    use crate::parser::common::error_fmt;

    impl<'a> std::fmt::Debug for Manifest<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Manifest::Image(image) => write!(f, "Manifest::Image({})", String::from_utf8_lossy(image)),
                Manifest::File(file) => write!(f, "Manifest::File({})", String::from_utf8_lossy(file)),
            }
        }
    }

    #[test]
    fn test_parser_container_image() {
        let input = b"from: ubuntu:latest\0";

        let result = manifest(input);

        assert!(result.is_ok(), format!("Error: {:?}", result.err().map(error_fmt)));
        let (_, manifest) = result.unwrap();
        assert_eq!(manifest, Manifest::Image(b"ubuntu:latest"))
    }

    #[test]
    fn test_parser_container_manifest() {
        let input = b"from: Dockerfile.ubuntu\0";

        let result = manifest(input);

        assert!(result.is_ok(), format!("Error: {:?}", result.err().map(error_fmt)));
        let (_, manifest) = result.unwrap();
        assert_eq!(manifest, Manifest::File(b"Dockerfile.ubuntu"))
    }

    #[test]
    fn test_parser_invalid_container_manifest() {
        let input = b"from: Dockerfile@ubuntu\0";

        let result = manifest(input);

        assert!(result.is_err());
    }
}
