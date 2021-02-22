use super::{Argument, space, line_feed};

named!(pub(in super) volume_from<Argument>,
    do_parse!(
        tag!(":") >>
        space >>
        name: terminated!(Argument::parse_volume_name, line_feed) >> (
            Argument::VolumeFrom {
                name
            }
        )
    )
);

impl Argument<'_> {
    fn parse_volume_name<T, E: nom::error::ParseError<T>>(input: T) -> nom::IResult<T, T, E>
        where T: nom::InputTakeAtPosition,
              <T as nom::InputTakeAtPosition>::Item: nom::AsChar,
    {
        use nom::AsChar;

        input.split_at_position1_complete(
            |item| match item.as_char() {
                '_' | '-' => false,
                chr if chr.is_alphanumeric() => false,
                _ => true
            },
            nom::error::ErrorKind::AlphaNumeric
        )
    }
}
