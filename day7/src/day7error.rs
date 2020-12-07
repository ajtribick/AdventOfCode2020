use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Day7Error {
    IoError(io::Error),
    ParseError,
}

impl fmt::Display for Day7Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Day7Error::IoError(e) => write!(f, "IoError({})", e),
            Day7Error::ParseError => write!(f, "Parse error"),
        }
    }
}

impl Error for Day7Error {}
