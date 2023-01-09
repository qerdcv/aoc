use std::{fmt, io};
use std::num::ParseIntError;

#[derive(Debug)]
pub struct Error {
    message: String
}

impl Error {
    pub fn new(message: String) -> Self {
        return Error{message}
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::new(value.to_string())
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::new(value.to_string())
    }
}

impl From<regex::Error> for Error {
    fn from(value: regex::Error) -> Self {
        Error::new(value.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
