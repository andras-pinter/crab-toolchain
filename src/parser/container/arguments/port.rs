use super::{Argument, space, digit, line_feed};

named!(pub(in super) port<Argument>,
    do_parse!(
        tag!(":") >>
        space >>
        outer: map_res!(take_until!(":"), Argument::parse_to_u16) >>
        tag!(":") >>
        inner: map_res!(terminated!(digit, line_feed), Argument::parse_to_u16) >> (
            Argument::PublishPort {
                outer,
                inner
            }
        )
    )
);
