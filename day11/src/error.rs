use std::{error::Error, fmt};

#[derive(Debug)]
pub enum Day11Error {
    ParseError(&'static str),
}

impl fmt::Display for Day11Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Day11Error::ParseError(d) => write!(f, "Parse error ({})", d),
        }
    }
}

impl Error for Day11Error {}
