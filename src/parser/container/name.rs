named!(pub container_name<&[u8]>,
    do_parse!(
        name: verify!(
            take_until!(":"),
            |name: &[u8]| name.iter().all(
                |chr| nom::character::is_alphanumeric(*chr) || b"_-".contains(chr)
            )
        ) >>
        tag!(":") >> (name)
    )
);

#[cfg(test)]
mod tests {
    use super::container_name;
    use crate::parser::common::error_fmt;

    #[test]
    fn test_parser_container_name() {
        let input = b"ubuntu:";

        let result = container_name(input);

        assert!(result.is_ok(), format!("Error: {:?}", result.err().map(error_fmt)));
        let (_, container_name) = result.unwrap();
        assert_eq!(container_name, b"ubuntu");
    }

    #[test]
    fn test_parser_container_custom_name() {
        let input = b"ubuntu-bionic:";

        let result = container_name(input);

        assert!(result.is_ok(), format!("Error: {:?}", result.err().map(error_fmt)));
        let (_, container_name) = result.unwrap();
        assert_eq!(container_name, b"ubuntu-bionic");
    }
}
