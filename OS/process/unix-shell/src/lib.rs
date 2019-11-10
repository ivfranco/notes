use std::io;

pub enum Error {
    IOError(io::Error),
    NixError(nix::Error),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Error::*;
        match self {
            IOError(err) => write!(f, "{}", err),
            NixError(err) => write!(f, "{}", err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IOError(err)
    }
}

impl From<nix::Error> for Error {
    fn from(err: nix::Error) -> Self {
        Error::NixError(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
