#![deny(missing_docs)]

//! A naive HTTP proxy, disregarding most RFCs.

pub mod cache;
pub mod http;
pub mod resolver;
pub mod server;

use chrono::{
    offset::{FixedOffset, Utc},
    DateTime,
};
use httparse::Error as ParseError;
use trust_dns_resolver::error::ResolveError;

/// Crate universal error type.
#[derive(Debug)]
pub enum Error {
    /// No body found for HTTP request or response
    BodyNotPresent,
    /// Format error in HTTP request or response
    MalformedHTTP,
    /// Proxy cannot handle this method
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

/// Internet time defined by RFC2822 in GMT timezone
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct GMTDateTime {
    inner: DateTime<FixedOffset>,
}

impl GMTDateTime {
    /// Return current datetime in GMT (UTC) timezone.
    pub fn now() -> Self {
        let inner = Utc::now().with_timezone(&FixedOffset::east(0));
        Self { inner }
    }

    /// Parse GMT date time from the format defined in RFC2822.
    pub fn parse_from_rfc2822(s: &str) -> Option<Self> {
        let inner = DateTime::parse_from_rfc2822(s).ok()?;
        Some(Self { inner })
    }

    /// Format date time in format defined in RFC2822.
    pub fn to_rfc2822(&self) -> String {
        self.inner.to_rfc2822()
    }
}