use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyStringArgument(String),
    FileCreationFailed(String),
}

impl Error {
    pub fn to_string(&self) -> String {
        match self {
            Error::EmptyStringArgument(argument) => format!("empty string: {}", argument),
            Error::FileCreationFailed(filename) => format!("file creation failed: {}", filename),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
