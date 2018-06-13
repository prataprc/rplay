extern crate getopts;

use getopts::Fail;
use std::error::{self, Error as ErrorTrait};
use std::fmt::{self,Display,Debug};

pub enum Error {
    GetoptsFail(Fail),
    InvalidMode(String),
}

impl From<Fail> for Error {
    fn from(fail: Fail) -> Error {
        Error::GetoptsFail(fail)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::GetoptsFail(_) => "getopts parse failed",
            Error::InvalidMode(_) => "invalid mode argument"
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            Error::GetoptsFail(fail) => Some(fail),
            Error::InvalidMode(_) => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cause = match self {
            Error::GetoptsFail(fail) => format!("{:?}", fail),
            Error::InvalidMode(mode) => format!("'{}'", mode),
        };
        write!(f, "{}, {}", self.description(), cause)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}
