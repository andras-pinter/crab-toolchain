pub trait PathLike {
    fn is_allowed(chr: char) -> bool {
        match chr {
            '/' | '_' | '-' => true,
            chr if chr.is_alphanumeric() => true,
            _ => false
        }
    }

    fn parse_path<T, E: nom::error::ParseError<T>>(input: T) -> nom::IResult<T, T, E>
        where T: nom::InputTakeAtPosition,
              <T as nom::InputTakeAtPosition>::Item: nom::AsChar,
    {
        use nom::AsChar;

        input.split_at_position1_complete(
            |item| !Self::is_allowed(item.as_char()),
            nom::error::ErrorKind::AlphaNumeric
        )
    }
}