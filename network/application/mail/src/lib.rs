pub mod client;
mod resolver;

use std::{
    io,
    path::PathBuf,
};
use trust_dns_resolver::error::ResolveError;

#[derive(Debug)]
pub enum MailError {
    IOError(io::Error),
    ResolveError(ResolveError),
    DomainNotFound,
    MalformedInput,
    MalfomredResponse,
    NegativeReply,
}

use MailError::*;

pub type Result<T> = std::result::Result<T, MailError>;

impl From<io::Error> for MailError {
    fn from(err: io::Error) -> Self {
        IOError(err)
    }
}

impl From<ResolveError> for MailError {
    fn from(err: ResolveError) -> Self {
        ResolveError(err)
    }
}

#[derive(Debug)]
pub struct Mail {
    pub send: String,
    pub from: String,
    pub recv: String,
    pub to: String,
    pub body: String,
    pub image: Option<PathBuf>,
}
