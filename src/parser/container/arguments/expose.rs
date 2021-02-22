use super::{Argument, space, digit, line_feed};

named!(pub(in super) expose<Argument>,
    do_parse!(
        tag!(":") >>
        space >>
        port: map_res!(terminated!(digit, line_feed), Argument::parse_to_u16) >> (
            Argument::ExposePort {
                port
            }
        )
    )
);
