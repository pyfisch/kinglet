use std::convert::From;
use std::str::Utf8Error;
use std::num::ParseIntError;

use url::ParseError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    InvalidMethod,
    InvalidVersion,
    InvalidMessage,
    ForbiddenHeader,
    MissingHeader,
    InvalidHeader,
    UrlError(ParseError),
    Utf8Error(Utf8Error),
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Error::UrlError(err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error::Utf8Error(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Error::InvalidHeader
    }
}
