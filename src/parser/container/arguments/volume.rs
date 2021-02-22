use super::{Argument, space, line_feed};
use crate::parser::PathLike;

named!(pub(in super) volume<Argument>,
    do_parse!(
        tag!(":") >>
        space >>
        source: verify!(take_until!(":"), Argument::verify_path) >>
        tag!(":") >>
        mount: terminated!(Argument::parse_path, line_feed) >> (
            Argument::Volume {
                source,
                mount
            }
        )
    )
);

impl Argument<'_> {
    fn verify_path(path: &[u8]) -> bool {
        path.iter().all(|chr| Self::is_allowed(*chr as char))
    }
}

impl PathLike for Argument<'_> {}
