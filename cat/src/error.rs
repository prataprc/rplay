extern crate getopts;

use std::io;
use std::convert::{From};

#[derive(Debug)]
pub enum Error {
    GetoptsErr(getopts::Fail),
    IOErr(io::Error),
}

impl From<getopts::Fail> for Error {
    fn from(e: getopts::Fail) -> Self { Error::GetoptsErr(e) }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self { Error::IOErr(e) }
}


