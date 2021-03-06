extern crate getopts;

use getopts::Fail;
use std::io::Error as IOError;
use std::error::{self, Error as ErrorTrait};
use std::fmt::{self,Display,Debug};

pub enum Error {
    GetoptsFail(Fail),
    InvalidMode(String),
    IO(IOError),
}

impl From<Fail> for Error {
    fn from(fail: Fail) -> Error {
        Error::GetoptsFail(fail)
    }
}

impl From<IOError> for Error {
    fn from(err: IOError) -> Error {
        Error::IO(err)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::GetoptsFail(fail) => fail.description(),
            Error::InvalidMode(_) => "invalid mode argument",
            Error::IO(err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            Error::GetoptsFail(fail) => Some(fail),
            Error::InvalidMode(_) => None,
            Error::IO(err) => Some(err),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cause = match self {
            Error::GetoptsFail(fail) => format!("{:?}", fail),
            Error::InvalidMode(mode) => format!("'{}'", mode),
            Error::IO(err) => format!("'{}'", err.description()),
        };
        write!(f, "{}, {}", self.description(), cause)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}
