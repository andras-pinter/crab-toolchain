use crate::parser::common::error_fmt;
use super::{argument, Argument};

impl<'a> std::fmt::Debug for Argument<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Argument::Volume { source, mount} => {
                let source = String::from_utf8_lossy(source);
                let mount = String::from_utf8_lossy(mount);
                writeln!(f, "Argument::Volume {{ source: {}, mount: {} }}", source, mount)
            }
            Argument::PublishPort { inner, outer } => {
                writeln!(f, "Argument::PublishPort {{ inner: {}, outer: {} }}", inner, outer)
            }
            Argument::ExposePort { port } => {
                writeln!(f, "Argument::Port {{ port: {} }}", port)
            }
            Argument::VolumeFrom { name } => {
                writeln!(f, "Argument::VolumeFrom {{ name: {} }}", String::from_utf8_lossy(name))
            }
        }
    }
}

#[test]
fn test_parse_invalid() {
    let input = b"invalid: invalid\0";

    let result = argument(input);

    assert!(result.is_err());
}

mod test_volume {
    use super::*;

    #[test]
    fn test_parse() {
        let input = b"volume: /path/to/directory:/path/to/mount/point\0";

        let result = argument(input);

        assert!(result.is_ok(), format!("Error: {:?}", result.err().map(error_fmt)));
        let (_, argument) = result.unwrap();
        assert_eq!(argument, Argument::Volume {
            source: b"/path/to/directory",
            mount: b"/path/to/mount/point"
        });
    }

    #[test]
    fn test_parse_invalid_mount_path() {
        let input = b"volume: /path/to/directory:/path%to|mount=point\0";

        let result = argument(input);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_source_path() {
        let input = b"volume: /path+to@directory:/path/to/mount/point\0";

        let result = argument(input);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_path() {
        let input = b"volume: /path+to@directory:!path%to|mount=point\0";

        let result = argument(input);

        assert!(result.is_err());
    }
}

mod test_publish_port {
    use super::*;

    #[test]
    fn test_parse() {
        let input = b"port: 80:8080\0";

        let result = argument(input);

        assert!(result.is_ok(), format!("Error: {:?}", result.err().map(error_fmt)));
        let (_, argument) = result.unwrap();
        assert_eq!(argument, Argument::PublishPort {
            outer: 80,
            inner: 8080
        });
    }

    #[test]
    fn test_parse_invalid_outer_port() {
        let input = b"port: abc:8080\0";

        let result = argument(input);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_inner_port() {
        let input = b"port: 80:abc\0";

        let result = argument(input);

        assert!(result.is_err());
    }
}

mod test_exposed_port {
    use super::*;

    #[test]
    fn test_parse() {
        let input = b"expose: 8080\0";

        let result = argument(input);

        assert!(result.is_ok(), format!("Error: {:?}", result.err().map(error_fmt)));
        let (_, argument) = result.unwrap();
        assert_eq!(argument, Argument::ExposePort {
            port: 8080
        });
    }

    #[test]
    fn test_parse_invalid_port() {
        let input = b"expose: abcd\0";

        let result = argument(input);

        assert!(result.is_err());
    }
}

mod test_volume_from {
    use super::*;

    #[test]
    fn test_parse() {
        let input = b"volume-from: cache_container\0";

        let result = argument(input);

        assert!(result.is_ok(), format!("Error: {:?}", result.err().map(error_fmt)));
        let (_, argument) = result.unwrap();
        assert_eq!(argument, Argument::VolumeFrom {
            name: b"cache_container"
        });
    }

    #[test]
    fn test_parse_invalid_volume_from() {
        let input = b"volume-from: cache@Container\0";

        let result = argument(input);
        assert!(result.is_err());
    }
}
