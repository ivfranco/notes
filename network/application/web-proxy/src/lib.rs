pub mod resolver;
pub mod server;

use httparse::Error as ParseError;
use trust_dns_resolver::error::ResolveError;

#[derive(Debug)]
pub enum Error {
    MalformedHTTP,
    MethodNotImplemented,
    ParseError(ParseError),
    ResolveError(ResolveError),
    IOError(std::io::Error),
}

use Error::*;

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        ParseError(err)
    }
}

impl From<ResolveError> for Error {
    fn from(err: ResolveError) -> Self {
        ResolveError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        IOError(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
