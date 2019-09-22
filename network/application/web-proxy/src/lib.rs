#![deny(missing_docs)]

//! A naive HTTP proxy, disregarding most RFCs.

pub mod http;
pub mod resolver;
pub mod server;

use httparse::Error as ParseError;
use trust_dns_resolver::error::ResolveError;

/// Crate universal error type.
#[derive(Debug)]
pub enum Error {
    /// No body found for HTTP request or response
    BodyNotPresent,
    /// Format error in HTTP request or response
    MalformedHTTP,
    ///
    MethodNotImplemented,
    /// Error propagated from httparse
    ParseError(ParseError),
    /// Error propagated from trust-dns-resolver
    ResolveError(ResolveError),
    /// Error propagated from std::io
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

/// Crate universal result type.
pub type Result<T> = std::result::Result<T, Error>;
