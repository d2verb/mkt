use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    InvalidArgument,
    FileCreationFailed,
}

impl ErrorKind {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::InvalidArgument => "invalid argument",
            ErrorKind::FileCreationFailed => "file creation failed",
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: Option<String>,
}

impl Error {
    pub fn new(kind: ErrorKind, message: Option<&str>) -> Error {
        return match message {
            Some(message) => Error {
                kind: kind,
                message: Some(message.to_string()),
            },
            None => Error {
                kind: kind,
                message: None,
            },
        };
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted = match (&self.kind, &self.message) {
            (_, Some(message)) => format!("{}: {}", self.kind.as_str(), message),
            (_, None) => self.kind.to_string(),
        };
        write!(f, "{}", formatted)
    }
}
