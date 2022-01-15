use std::{error, fmt, num};

pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Error { kind }
    }

    #[allow(dead_code)]
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl error::Error for Error {}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::InvalidInputExpression => {
                write!(f, "<input-expression> is invalid")
            }
            ErrorKind::InvalidNumberInExpression => {
                write!(f, "<input-expression> contains an invalid number")
            }
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::InvalidInputExpression => {
                write!(f, "<input-expression> is invalid")
            }
            ErrorKind::InvalidNumberInExpression => {
                write!(f, "<input-expression> contains an invalid number")
            }
        }
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(_: num::ParseFloatError) -> Self {
        Error { kind: ErrorKind::InvalidNumberInExpression }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidInputExpression,
    InvalidNumberInExpression,
}
