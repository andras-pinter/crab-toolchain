mod volume;
mod port;
mod expose;
mod volume_from;
#[cfg(test)]
mod tests;

use super::{space, digit, line_feed};

named!(pub argument<Argument>,
    do_parse!(
        arg: switch!(take_until!(":"),
            b"volume" => call!(volume::volume) |
            b"port" => call!(port::port) |
            b"expose" => call!(expose::expose) |
            b"volume-from" => call!(volume_from::volume_from)
        ) >> (arg)
    )
);

#[derive(Eq, PartialEq, Hash)]
pub enum Argument<'a> {
    Volume {
        source: &'a [u8],
        mount: &'a [u8]
    },
    PublishPort {
        outer: u16,
        inner: u16
    },
    ExposePort {
        port: u16
    },
    VolumeFrom {
        name: &'a [u8]
    }
}

impl<'a> Argument<'a> {
    fn parse_to_u16(bytes: &[u8]) -> Result<u16, std::num::ParseIntError> {
        String::from_utf8_lossy(bytes).parse::<u16>()
    }
}
