mod container;
mod shell;
mod path;
#[cfg(test)]
mod tests;

use path::PathLike;
pub(in crate::parser) use nom::character::complete::{
    line_ending as newline,
    digit1 as digit,
};
use std::collections::HashMap;
use shell::{shell, Shell};
use container::{container, Container};

named!(pub(in crate::parser) space<char>, char!(' '));
named!(pub(in crate::parser) tab, alt!(tag!("\t") | tag!("    ")));
named!(pub(in crate::parser) line_feed, alt!(newline | tag!("\0")));

named!(parse<Parser>,
    do_parse!(
        shell: opt!(terminated!(shell, many0!(newline))) >>
        containers: many0!(complete!(terminated!(container, many0!(newline)))) >>
        alt!(newline | eof!()) >> (
            Parser {
                shell: shell.unwrap_or_default(),
                containers: containers.into_iter().collect()
            }
        )
    )
);

#[cfg_attr(test, derive(Debug))]
pub struct Parser<'a> {
    shell: Shell<'a>,
    containers: HashMap<&'a [u8], Container<'a>>
}

impl<'a> Parser<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, nom::Err<nom::error::Error<&'a [u8]>>> {
        parse(input).map(|res| res.1)
    }
}

#[cfg(test)]
pub(in crate::parser) mod common {
    type NomError<I> = nom::Err<nom::error::Error<I>>;
    type NomErrorFmt<'a> = nom::Err<(nom::error::ErrorKind, std::borrow::Cow<'a, str>)>;

    pub fn error_fmt(err: NomError<&[u8]>) -> NomErrorFmt {
        err.map(|e| (e.code, String::from_utf8_lossy(e.input)))
    }
}
